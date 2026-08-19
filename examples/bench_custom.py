#!/usr/bin/env python3
"""Matched custom-graph evolution bench for legacy GOLEM.

Mirrors examples/bench_custom.rs and GOLEM/test/unit/test_custom.py.
Writes the same JSON schema so reports can be compared with:

    cargo run -p irongolem-bench --release -- --compare \\
        examples/results/python_custom.json examples/results/rust_custom.json
"""

from __future__ import annotations

import argparse
import json
import math
import statistics
import sys
import time
from datetime import timedelta
from pathlib import Path

SCHEMA = "irongolem.bench.custom.v1"
REPO_ROOT = Path(__file__).resolve().parents[1]
GOLEM_ROOT = REPO_ROOT / "GOLEM"


def _ensure_golem_on_path() -> None:
    if str(GOLEM_ROOT) not in sys.path:
        sys.path.insert(0, str(GOLEM_ROOT))


def mean(xs):
    return statistics.fmean(xs) if xs else None


def stddev(xs):
    if not xs:
        return None
    if len(xs) < 2:
        return 0.0
    return statistics.stdev(xs)


def summarise(trials):
    ok = [t for t in trials if t.get("evaluated_ok")]
    walls = [t["wall_seconds"] for t in ok]
    fits = [t["best_fitness"][0] for t in ok if t.get("best_fitness")]
    sizes = [float(t["best_graph_size"]) for t in ok]
    return {
        "n_trials": len(trials),
        "n_ok": len(ok),
        "mean_wall_seconds": mean(walls),
        "std_wall_seconds": stddev(walls),
        "mean_best_fitness": mean(fits),
        "std_best_fitness": stddev(fits),
        "mean_best_graph_size": mean(sizes),
        "std_best_graph_size": stddev(sizes),
    }


def run_trial(params, seed: int):
    _ensure_golem_on_path()
    try:
        from golem.core.adapter import DirectAdapter
        from golem.core.dag.verification_rules import has_no_self_cycled_nodes
        from golem.core.optimisers.genetic.gp_optimizer import EvoGraphOptimizer
        from golem.core.optimisers.genetic.gp_params import GPAlgorithmParameters
        from golem.core.optimisers.genetic.operators.base_mutations import MutationTypesEnum
        from golem.core.optimisers.genetic.operators.regularization import RegularizationTypesEnum
        from golem.core.optimisers.initial_graphs_generator import InitialPopulationGenerator
        from golem.core.optimisers.objective.objective import Objective
        from golem.core.optimisers.objective.objective_eval import ObjectiveEvaluate
        from golem.core.optimisers.opt_node_factory import DefaultOptNodeFactory
        from golem.core.optimisers.optimization_parameters import GraphRequirements
        from golem.core.optimisers.optimizer import GraphGenerationParams
        from golem.utilities.utilities import set_random_seed
        from test.unit.test_custom import CustomModel, CustomNode, custom_metric
        from test.unit.utils import graph_fifth, graph_first, graph_fourth, graph_second, graph_third
    except ImportError as exc:
        return {
            "seed": seed,
            "wall_seconds": 0.0,
            "generations": 0,
            "best_fitness": [],
            "best_graph_size": 0,
            "evaluated_ok": False,
            "error": (
                f"{exc}. Install GOLEM and deps from the repo root: "
                f'pip install -e "{GOLEM_ROOT}"'
            ),
        }

    set_random_seed(seed)

    requirements = GraphRequirements(
        num_of_generations=params["num_of_generations"],
        show_progress=False,
        n_jobs=params["n_jobs"],
        keep_history=True,
        early_stopping_iterations=max(params["num_of_generations"] * 10, 1000),
        early_stopping_timeout=1e9,
        timeout=(
            timedelta(seconds=params["timeout_seconds"])
            if params.get("timeout_seconds")
            else None
        ),
        parallelization_mode="sequential" if params["n_jobs"] == 1 else "populational",
        history_dir=None,
    )

    optimiser_parameters = GPAlgorithmParameters(
        pop_size=params["pop_size"],
        mutation_types=[
            MutationTypesEnum.simple,
            MutationTypesEnum.reduce,
            MutationTypesEnum.growth,
            MutationTypesEnum.local_growth,
        ],
        regularization_type=RegularizationTypesEnum.none,
    )
    optimiser_parameters.seed = seed

    graph_generation_params = GraphGenerationParams(
        adapter=DirectAdapter(base_graph_class=CustomModel, base_node_class=CustomNode),
        rules_for_constraint=[has_no_self_cycled_nodes],
        node_factory=DefaultOptNodeFactory(available_node_types=["A", "B", "C", "D"]),
    )

    objective = Objective({"custom": custom_metric})
    initial_graphs = [
        graph_first(),
        graph_second(),
        graph_third(),
        graph_fourth(),
        graph_fifth(),
    ]
    init_population = InitialPopulationGenerator(
        optimiser_parameters.pop_size,
        graph_generation_params,
        requirements,
    ).with_initial_graphs(initial_graphs)()

    optimiser = EvoGraphOptimizer(
        graph_generation_params=graph_generation_params,
        objective=objective,
        graph_optimizer_params=optimiser_parameters,
        requirements=requirements,
        initial_graphs=init_population,
    )
    objective_eval = ObjectiveEvaluate(objective)

    started = time.perf_counter()
    try:
        optimized_graphs = optimiser.optimise(objective_eval)
        wall_seconds = time.perf_counter() - started
        best = optimiser.generations.best_individuals
        if best:
            fitness_values = list(best[0].fitness.values)
            graph_size = best[0].graph.length
        elif optimized_graphs:
            restored = optimiser.graph_generation_params.adapter.restore(optimized_graphs[0])
            fitness_values = []
            graph_size = restored.length
        else:
            fitness_values = []
            graph_size = 0
        return {
            "seed": seed,
            "wall_seconds": wall_seconds,
            "generations": optimiser.current_generation_num,
            "best_fitness": fitness_values,
            "best_graph_size": int(graph_size),
            "evaluated_ok": bool(optimized_graphs),
            "error": None,
        }
    except Exception as exc:  # noqa: BLE001 — bench must record failures
        return {
            "seed": seed,
            "wall_seconds": time.perf_counter() - started,
            "generations": getattr(optimiser, "current_generation_num", 0),
            "best_fitness": [],
            "best_graph_size": 0,
            "evaluated_ok": False,
            "error": str(exc),
        }


def fmt_opt(v):
    if v is None or (isinstance(v, float) and not math.isfinite(v)):
        return "n/a"
    return f"{v:.4f}"


def compare_reports(left_path: Path, right_path: Path) -> None:
    left = json.loads(left_path.read_text())
    right = json.loads(right_path.read_text())
    print(f"{'metric':<22} {left['implementation']:>16} {right['implementation']:>16}")
    print("-" * 56)
    left_ok = f"{left['summary']['n_ok']}/{left['summary']['n_trials']}"
    right_ok = f"{right['summary']['n_ok']}/{right['summary']['n_trials']}"
    print(f"{'ok trials':<22} {left_ok:>16} {right_ok:>16}")
    for key, label in [
        ("mean_wall_seconds", "mean wall s"),
        ("std_wall_seconds", "std wall s"),
        ("mean_best_fitness", "mean best fitness"),
        ("std_best_fitness", "std best fitness"),
        ("mean_best_graph_size", "mean graph size"),
    ]:
        print(
            f"{label:<22} {fmt_opt(left['summary'].get(key)):>16} "
            f"{fmt_opt(right['summary'].get(key)):>16}"
        )
    a = left["summary"].get("mean_wall_seconds")
    b = right["summary"].get("mean_wall_seconds")
    if a and b and a > 0 and b > 0:
        if a < b:
            faster, slower, speedup = left["implementation"], right["implementation"], b / a
        else:
            faster, slower, speedup = right["implementation"], left["implementation"], a / b
        print()
        print(f"{faster} wall-clock is {speedup:.2f}× {slower} (equal generation budget).")
    print()
    print("Lower fitness is better (metric is -graph_length). Seeds and operators match; RNGs do not.")


def parse_args(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--trials", type=int, default=5)
    parser.add_argument("--generations", type=int, default=10)
    parser.add_argument("--pop-size", type=int, default=5)
    parser.add_argument("--n-jobs", type=int, default=1)
    parser.add_argument("--timeout-seconds", type=float, default=0.0)
    parser.add_argument("--seed-start", type=int, default=1)
    parser.add_argument(
        "--output",
        type=Path,
        default=Path("examples/results/python_custom.json"),
    )
    parser.add_argument("--compare", nargs=2, metavar=("A.json", "B.json"))
    return parser.parse_args(argv)


def main(argv=None):
    args = parse_args(argv)
    if args.compare:
        compare_reports(Path(args.compare[0]), Path(args.compare[1]))
        return 0

    if not GOLEM_ROOT.exists():
        print(
            f"GOLEM checkout not found at {GOLEM_ROOT}. "
            "Clone it with: git clone https://github.com/aimclub/GOLEM.git GOLEM",
            file=sys.stderr,
        )
        return 1

    _ensure_golem_on_path()
    try:
        import golem  # noqa: F401
        import networkx  # noqa: F401
    except ImportError as exc:
        print(
            f"Cannot import GOLEM ({exc}). From the repo root run:\n"
            f'  pip install -e "{GOLEM_ROOT}"',
            file=sys.stderr,
        )
        return 1

    params = {
        "pop_size": args.pop_size,
        "num_of_generations": args.generations,
        "n_jobs": args.n_jobs,
        "timeout_seconds": args.timeout_seconds if args.timeout_seconds > 0 else None,
        "seed_start": args.seed_start,
    }

    trials = []
    for k in range(args.trials):
        seed = args.seed_start + k
        print(f"golem trial {k + 1}/{args.trials} seed={seed}", file=sys.stderr)
        result = run_trial(params, seed)
        print(
            f"  wall={result['wall_seconds']:.3f}s gens={result['generations']} "
            f"fitness={result['best_fitness']} size={result['best_graph_size']} "
            f"ok={result['evaluated_ok']}",
            file=sys.stderr,
        )
        trials.append(result)

    report = {
        "schema": SCHEMA,
        "implementation": "python",
        "problem": "custom_graph",
        "params": params,
        "trials": trials,
        "summary": summarise(trials),
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"wrote {args.output}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
