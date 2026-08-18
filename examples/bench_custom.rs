//! Matched custom-graph evolution bench for irongolem.
//!
//! Mirrors `examples/bench_custom.py` and `test_custom_graph_opt`. Dumps JSON
//! with the same schema so Python and Rust trials can be compared.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use irongolem::golem::dag::{has_no_self_cycled_nodes, Graph};
use irongolem::golem::optimisers::genetic::operators::base_mutations::MutationTypesEnum;
use irongolem::golem::optimisers::genetic::params::{
    GPAlgorithmParameters, GraphGenerationParams, GraphRequirements,
};
use irongolem::golem::optimisers::genetic::rng::{set_random_seed, GeneticRng};
use irongolem::golem::optimisers::genetic::EvoGraphOptimizer;
use irongolem::golem::optimisers::initial_population_generator::InitialPopulationGenerator;
use irongolem::golem::optimisers::objective::ObjectiveEvaluate;
use serde::{Deserialize, Serialize};
use test_support::fixtures::{custom_initial_graphs, custom_objective, CustomDirectAdapter};

const SCHEMA: &str = "irongolem.bench.custom.v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchParams {
    pop_size: usize,
    num_of_generations: usize,
    n_jobs: i32,
    timeout_seconds: Option<f64>,
    seed_start: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrialResult {
    seed: u64,
    wall_seconds: f64,
    generations: usize,
    best_fitness: Vec<f64>,
    best_graph_size: usize,
    evaluated_ok: bool,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchSummary {
    n_trials: usize,
    n_ok: usize,
    mean_wall_seconds: Option<f64>,
    std_wall_seconds: Option<f64>,
    mean_best_fitness: Option<f64>,
    std_best_fitness: Option<f64>,
    mean_best_graph_size: Option<f64>,
    std_best_graph_size: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchReport {
    schema: String,
    implementation: String,
    problem: String,
    params: BenchParams,
    trials: Vec<TrialResult>,
    summary: BenchSummary,
}

struct Cli {
    trials: usize,
    generations: usize,
    pop_size: usize,
    n_jobs: i32,
    timeout_seconds: Option<f64>,
    seed_start: u64,
    output: PathBuf,
    compare: Option<(PathBuf, PathBuf)>,
}

impl Cli {
    fn parse() -> Result<Self, String> {
        let args: Vec<String> = std::env::args().skip(1).collect();
        if args.iter().any(|a| a == "-h" || a == "--help") {
            print_help();
            std::process::exit(0);
        }

        let mut trials = 5usize;
        let mut generations = 10usize;
        let mut pop_size = 5usize;
        let mut n_jobs = 1i32;
        let mut timeout_seconds = None;
        let mut seed_start = 1u64;
        let mut output = PathBuf::from("examples/results/rust_custom.json");
        let mut compare = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--trials" => {
                    trials = parse_next(&args, &mut i, "--trials")?;
                }
                "--generations" => {
                    generations = parse_next(&args, &mut i, "--generations")?;
                }
                "--pop-size" => {
                    pop_size = parse_next(&args, &mut i, "--pop-size")?;
                }
                "--n-jobs" => {
                    n_jobs = parse_next(&args, &mut i, "--n-jobs")?;
                }
                "--timeout-seconds" => {
                    let v: f64 = parse_next(&args, &mut i, "--timeout-seconds")?;
                    timeout_seconds = if v <= 0.0 { None } else { Some(v) };
                }
                "--seed-start" => {
                    seed_start = parse_next(&args, &mut i, "--seed-start")?;
                }
                "--output" => {
                    output = PathBuf::from(require_next(&args, &mut i, "--output")?);
                }
                "--compare" => {
                    let a = require_next(&args, &mut i, "--compare")?;
                    i += 1;
                    let b = args
                        .get(i)
                        .cloned()
                        .ok_or_else(|| "--compare needs two JSON paths".to_string())?;
                    compare = Some((PathBuf::from(a), PathBuf::from(b)));
                }
                other => return Err(format!("unknown argument: {other}")),
            }
            i += 1;
        }

        Ok(Self {
            trials,
            generations,
            pop_size,
            n_jobs,
            timeout_seconds,
            seed_start,
            output,
            compare,
        })
    }
}

fn parse_next<T: std::str::FromStr>(args: &[String], i: &mut usize, flag: &str) -> Result<T, String>
where
    T::Err: std::fmt::Display,
{
    let raw = require_next(args, i, flag)?;
    raw.parse::<T>()
        .map_err(|e| format!("invalid {flag} value `{raw}`: {e}"))
}

fn require_next(args: &[String], i: &mut usize, flag: &str) -> Result<String, String> {
    *i += 1;
    args.get(*i)
        .cloned()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn print_help() {
    eprintln!(
        "\
Usage:
  cargo run -p irongolem-bench --release -- [options]
  cargo run -p irongolem-bench --release -- --compare a.json b.json

Options:
  --trials N             number of independent seeds (default: 5)
  --generations N        evolution generations (default: 10)
  --pop-size N           population size (default: 5)
  --n-jobs N             1 = sequential, -1 = all cores (default: 1)
  --timeout-seconds S    wall-clock cap; 0 disables (default: disabled)
  --seed-start N         first seed; trials use N, N+1, ... (default: 1)
  --output PATH          JSON report path (default: examples/results/rust_custom.json)
  --compare A.json B.json
                         print a side-by-side summary of two reports
"
    );
}

fn mean(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        None
    } else {
        Some(xs.iter().sum::<f64>() / xs.len() as f64)
    }
}

fn stddev(xs: &[f64]) -> Option<f64> {
    let m = mean(xs)?;
    if xs.len() < 2 {
        return Some(0.0);
    }
    let var = xs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (xs.len() as f64 - 1.0);
    Some(var.sqrt())
}

fn summarise(trials: &[TrialResult]) -> BenchSummary {
    let ok: Vec<&TrialResult> = trials.iter().filter(|t| t.evaluated_ok).collect();
    let walls: Vec<f64> = ok.iter().map(|t| t.wall_seconds).collect();
    let fits: Vec<f64> = ok
        .iter()
        .map(|t| t.best_fitness.first().copied().unwrap_or(f64::NAN))
        .collect();
    let sizes: Vec<f64> = ok.iter().map(|t| t.best_graph_size as f64).collect();
    BenchSummary {
        n_trials: trials.len(),
        n_ok: ok.len(),
        mean_wall_seconds: mean(&walls),
        std_wall_seconds: stddev(&walls),
        mean_best_fitness: mean(&fits),
        std_best_fitness: stddev(&fits),
        mean_best_graph_size: mean(&sizes),
        std_best_graph_size: stddev(&sizes),
    }
}

fn run_trial(params: &BenchParams, seed: u64) -> TrialResult {
    set_random_seed(seed);

    let nodes_types = ["A", "B", "C", "D"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

    let requirements = GraphRequirements {
        num_of_generations: Some(params.num_of_generations),
        show_progress: false,
        n_jobs: params.n_jobs,
        keep_history: true,
        early_stopping_iterations: Some(params.num_of_generations.saturating_mul(10).max(1_000)),
        early_stopping_timeout: Some(f64::MAX / 4.0),
        timeout: params
            .timeout_seconds
            .map(std::time::Duration::from_secs_f64),
        ..GraphRequirements::default()
    };

    let optimiser_parameters = GPAlgorithmParameters::new(params.pop_size)
        .with_mutation_types(vec![
            MutationTypesEnum::Simple,
            MutationTypesEnum::Reduce,
            MutationTypesEnum::Growth,
            MutationTypesEnum::LocalGrowth,
        ])
        .with_random_seed(seed);

    let mut graph_generation_params =
        GraphGenerationParams::new(nodes_types).with_rng(GeneticRng::seeded(seed));
    graph_generation_params.verifier = Arc::new(|graph| has_no_self_cycled_nodes(graph).is_ok());

    let adapter = CustomDirectAdapter;
    let objective = custom_objective();
    let initial_graphs = custom_initial_graphs();
    let init_population = InitialPopulationGenerator::new(
        optimiser_parameters.pop_size,
        graph_generation_params.clone(),
        requirements.clone(),
    )
    .with_initial_graphs(
        initial_graphs
            .into_iter()
            .map(|m| adapter.adapt(m))
            .collect(),
    )
    .generate();

    let mut optimiser = EvoGraphOptimizer::new(
        objective.clone(),
        Some(init_population),
        requirements,
        graph_generation_params,
        optimiser_parameters,
    );
    let objective_eval = ObjectiveEvaluate::new(objective);

    let started = Instant::now();
    match optimiser.optimise(&objective_eval) {
        Ok(graphs) => {
            let wall_seconds = started.elapsed().as_secs_f64();
            let best = optimiser.populational.best_individuals();
            let (best_fitness, best_graph_size) = if let Some(ind) = best.first() {
                (ind.fitness.values(), ind.graph.length())
            } else if let Some(graph) = graphs.first() {
                (Vec::new(), graph.length())
            } else {
                (Vec::new(), 0)
            };
            TrialResult {
                seed,
                wall_seconds,
                generations: optimiser.populational.generations.generation_num(),
                best_fitness,
                best_graph_size,
                evaluated_ok: !graphs.is_empty(),
                error: None,
            }
        }
        Err(err) => TrialResult {
            seed,
            wall_seconds: started.elapsed().as_secs_f64(),
            generations: optimiser.populational.generations.generation_num(),
            best_fitness: Vec::new(),
            best_graph_size: 0,
            evaluated_ok: false,
            error: Some(err.to_string()),
        },
    }
}

fn fmt_opt(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.4}"),
        _ => "n/a".into(),
    }
}

fn compare_reports(left_path: &Path, right_path: &Path) -> Result<(), String> {
    let left: BenchReport = serde_json::from_str(
        &fs::read_to_string(left_path).map_err(|e| format!("read {}: {e}", left_path.display()))?,
    )
    .map_err(|e| format!("parse {}: {e}", left_path.display()))?;
    let right: BenchReport = serde_json::from_str(
        &fs::read_to_string(right_path)
            .map_err(|e| format!("read {}: {e}", right_path.display()))?,
    )
    .map_err(|e| format!("parse {}: {e}", right_path.display()))?;

    println!(
        "{:<22} {:>16} {:>16}",
        "metric", left.implementation, right.implementation
    );
    println!("{}", "-".repeat(56));
    println!(
        "{:<22} {:>16} {:>16}",
        "ok trials",
        format!("{}/{}", left.summary.n_ok, left.summary.n_trials),
        format!("{}/{}", right.summary.n_ok, right.summary.n_trials)
    );
    println!(
        "{:<22} {:>16} {:>16}",
        "mean wall s",
        fmt_opt(left.summary.mean_wall_seconds),
        fmt_opt(right.summary.mean_wall_seconds)
    );
    println!(
        "{:<22} {:>16} {:>16}",
        "std wall s",
        fmt_opt(left.summary.std_wall_seconds),
        fmt_opt(right.summary.std_wall_seconds)
    );
    println!(
        "{:<22} {:>16} {:>16}",
        "mean best fitness",
        fmt_opt(left.summary.mean_best_fitness),
        fmt_opt(right.summary.mean_best_fitness)
    );
    println!(
        "{:<22} {:>16} {:>16}",
        "std best fitness",
        fmt_opt(left.summary.std_best_fitness),
        fmt_opt(right.summary.std_best_fitness)
    );
    println!(
        "{:<22} {:>16} {:>16}",
        "mean graph size",
        fmt_opt(left.summary.mean_best_graph_size),
        fmt_opt(right.summary.mean_best_graph_size)
    );

    if let (Some(a), Some(b)) = (
        left.summary.mean_wall_seconds,
        right.summary.mean_wall_seconds,
    ) {
        if a > 0.0 && b > 0.0 {
            let (faster, slower, speedup) = if a < b {
                (&left.implementation, &right.implementation, b / a)
            } else {
                (&right.implementation, &left.implementation, a / b)
            };
            println!();
            println!("{faster} wall-clock is {speedup:.2}× {slower} (equal generation budget).");
        }
    }
    println!();
    println!(
        "Lower fitness is better (metric is -graph_length). Seeds and operators match; RNGs do not."
    );
    Ok(())
}

fn main() -> Result<(), String> {
    let cli = Cli::parse()?;
    if let Some((a, b)) = cli.compare {
        return compare_reports(&a, &b);
    }

    let params = BenchParams {
        pop_size: cli.pop_size,
        num_of_generations: cli.generations,
        n_jobs: cli.n_jobs,
        timeout_seconds: cli.timeout_seconds,
        seed_start: cli.seed_start,
    };

    let mut trials = Vec::with_capacity(cli.trials);
    for k in 0..cli.trials {
        let seed = cli.seed_start + k as u64;
        eprintln!("irongolem trial {}/{} seed={seed}", k + 1, cli.trials);
        let result = run_trial(&params, seed);
        eprintln!(
            "  wall={:.3}s gens={} fitness={:?} size={} ok={}",
            result.wall_seconds,
            result.generations,
            result.best_fitness,
            result.best_graph_size,
            result.evaluated_ok
        );
        trials.push(result);
    }

    let report = BenchReport {
        schema: SCHEMA.into(),
        implementation: "rust".into(),
        problem: "custom_graph".into(),
        params,
        summary: summarise(&trials),
        trials,
    };

    if let Some(parent) = cli.output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
        }
    }
    let json = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
    fs::write(&cli.output, json).map_err(|e| format!("write {}: {e}", cli.output.display()))?;
    eprintln!("wrote {}", cli.output.display());
    Ok(())
}
