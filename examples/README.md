# Matched custom-graph experiment for Python GOLEM vs Rust irongolem.
#
# Both sides evolve the same toy graphs (`graph_first` … `graph_fifth`) with
# metric `-graph_length` (lower is better). Operators, population size,
# generations, and seeds are aligned. RNGs are **not** identical, so compare
# distributions (mean ± std over trials), not individual graphs.
#
# Prerequisites
#   Python: local `GOLEM/` checkout and a venv (macOS blocks system pip):
#
#     python3 -m venv .venv && source .venv/bin/activate
#     pip install -e GOLEM
#
#   Rust:   `cargo build -p irongolem-bench --release`
#
# 1. Generation-capped run (fair quality comparison)
#
#   python examples/bench_custom.py \
#     --trials 20 --generations 50 --pop-size 20 --n-jobs 1 \
#     --output examples/results/python_custom.json
#
#   cargo run -p irongolem-bench --release -- \
#     --trials 20 --generations 50 --pop-size 20 --n-jobs 1 \
#     --output examples/results/rust_custom.json
#
# 2. Parallel follow-up (same budget, n_jobs=-1)
#    Repeat the commands with `--n-jobs -1` and different `--output` paths.
#
# 3. Compare reports
#
#   cargo run -p irongolem-bench --release -- --compare \
#     examples/results/python_custom.json examples/results/rust_custom.json
#
#   python examples/bench_custom.py --compare \
#     examples/results/python_custom.json examples/results/rust_custom.json
#
# JSON schema (`irongolem.bench.custom.v1`)
#   implementation, problem, params, trials[], summary
#   each trial: seed, wall_seconds, generations, best_fitness, best_graph_size, evaluated_ok
#
# Always use `--release` on the Rust side. Debug builds are not a fair speed test.
