use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use serde_json::json;

use super::fitness::Fitness;
use super::history::{GraphEvalResult, Individual};
use super::timer::OptimisationTimer;
use crate::golem::adapter::{DirectAdapter, OptimizationAdapter};
use crate::golem::dag::GraphDelegate;
use crate::golem::utilities::determine_n_jobs;

pub type ObjectiveFn = Arc<dyn Fn(Arc<GraphDelegate>) -> Fitness + Send + Sync>;
pub type Evaluator = Arc<dyn Fn(Vec<Individual>) -> Vec<Individual> + Send + Sync>;

pub trait EvaluationDispatcher {
    fn dispatch(&self, objective: ObjectiveFn, timer: Option<OptimisationTimer>) -> Evaluator;
}

/// Split individuals into those needing evaluation and those already valid.
pub fn split_individuals_to_evaluate(
    individuals: Vec<Individual>,
) -> (Vec<Individual>, Vec<Individual>) {
    let mut individuals_to_evaluate = Vec::new();
    let mut individuals_to_skip = Vec::new();
    for ind in individuals {
        if ind.fitness.is_valid() {
            individuals_to_skip.push(ind);
        } else {
            individuals_to_evaluate.push(ind);
        }
    }
    (individuals_to_evaluate, individuals_to_skip)
}

/// Apply successful evaluation results back onto individuals.
pub fn apply_evaluation_results(
    individuals: Vec<Individual>,
    evaluation_results: Vec<Option<GraphEvalResult>>,
) -> Vec<Individual> {
    let evaluation_map: HashMap<String, GraphEvalResult> = evaluation_results
        .into_iter()
        .flatten()
        .filter(|res| res.is_successful())
        .map(|res| (res.uid_of_individual.clone(), res))
        .collect();

    let mut individuals_evaluated = Vec::new();
    for mut ind in individuals {
        let Some(eval_res) = evaluation_map.get(&ind.uid) else {
            continue;
        };
        if ind.set_evaluation_result(eval_res.clone()).is_ok() {
            individuals_evaluated.push(ind);
        }
    }
    individuals_evaluated
}

fn prepare_timer(timer: Option<OptimisationTimer>) -> Arc<Mutex<OptimisationTimer>> {
    let mut t = timer.unwrap_or_else(OptimisationTimer::forever);
    if !t.is_started() {
        t.start();
    }
    Arc::new(Mutex::new(t))
}

fn evaluation_time_iso() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| format!("{}", d.as_secs_f64()))
        .unwrap_or_default()
}

fn evaluate_single(
    adapter: &Arc<dyn OptimizationAdapter>,
    objective: &ObjectiveFn,
    timer: &Arc<Mutex<OptimisationTimer>>,
    graph: Arc<GraphDelegate>,
    uid_of_individual: String,
    with_time_limit: bool,
) -> Option<GraphEvalResult> {
    if with_time_limit {
        let mut t = timer.lock().unwrap();
        if t.is_time_limit_reached(None) {
            return None;
        }
    }

    let start_time = Instant::now();
    let restored = adapter.restore_graph(graph);
    let fitness = objective(Arc::new(restored.clone()));
    let elapsed = start_time.elapsed().as_secs_f64();

    let mut metadata = HashMap::new();
    metadata.insert("computation_time_in_seconds".into(), json!(elapsed));
    metadata.insert("evaluation_time_iso".into(), json!(evaluation_time_iso()));

    Some(GraphEvalResult {
        uid_of_individual,
        fitness,
        graph: Arc::new(restored),
        metadata,
    })
}

fn evaluate_population_sequential(
    adapter: Arc<dyn OptimizationAdapter>,
    objective: ObjectiveFn,
    timer: Arc<Mutex<OptimisationTimer>>,
    population: Vec<Individual>,
) -> Vec<Individual> {
    let (individuals_to_evaluate, individuals_to_skip) = split_individuals_to_evaluate(population);

    let evaluation_results: Vec<Option<GraphEvalResult>> = individuals_to_evaluate
        .iter()
        .map(|ind| {
            evaluate_single(
                &adapter,
                &objective,
                &timer,
                ind.graph.clone(),
                ind.uid.clone(),
                true,
            )
        })
        .collect();

    let mut individuals_evaluated =
        apply_evaluation_results(individuals_to_evaluate, evaluation_results);
    individuals_evaluated.extend(individuals_to_skip);
    individuals_evaluated
}

fn evaluate_population_parallel(
    adapter: Arc<dyn OptimizationAdapter>,
    objective: ObjectiveFn,
    timer: Arc<Mutex<OptimisationTimer>>,
    n_jobs: i32,
    population: Vec<Individual>,
) -> Vec<Individual> {
    let (individuals_to_evaluate, individuals_to_skip) =
        split_individuals_to_evaluate(population.clone());

    let n_threads = determine_n_jobs(n_jobs).unwrap_or(1);
    let pool = ThreadPoolBuilder::new()
        .num_threads(n_threads)
        .build()
        .expect("rayon thread pool");

    let evaluation_results: Vec<Option<GraphEvalResult>> = pool.install(|| {
        individuals_to_evaluate
            .par_iter()
            .map(|ind| {
                evaluate_single(
                    &adapter,
                    &objective,
                    &timer,
                    ind.graph.clone(),
                    ind.uid.clone(),
                    true,
                )
            })
            .collect()
    });

    let individuals_evaluated =
        apply_evaluation_results(individuals_to_evaluate, evaluation_results);
    let mut successful_evals = individuals_evaluated;
    successful_evals.extend(individuals_to_skip);

    if successful_evals.is_empty() {
        for single_ind in population {
            if let Some(result) = evaluate_single(
                &adapter,
                &objective,
                &timer,
                single_ind.graph.clone(),
                single_ind.uid.clone(),
                false,
            ) {
                let applied = apply_evaluation_results(vec![single_ind], vec![Some(result)]);
                if !applied.is_empty() {
                    successful_evals = applied;
                    break;
                }
            }
        }
    }

    successful_evals
}

#[derive(Clone)]
pub struct SequentialDispatcher {
    pub adapter: Arc<dyn OptimizationAdapter>,
}

impl Default for SequentialDispatcher {
    fn default() -> Self {
        Self {
            adapter: Arc::new(DirectAdapter),
        }
    }
}

impl SequentialDispatcher {
    pub fn new(adapter: Arc<dyn OptimizationAdapter>) -> Self {
        Self { adapter }
    }
}

impl EvaluationDispatcher for SequentialDispatcher {
    fn dispatch(&self, objective: ObjectiveFn, timer: Option<OptimisationTimer>) -> Evaluator {
        let adapter = self.adapter.clone();
        let timer = prepare_timer(timer);
        Arc::new(move |population| {
            evaluate_population_sequential(
                adapter.clone(),
                objective.clone(),
                timer.clone(),
                population,
            )
        })
    }
}

#[derive(Clone)]
pub struct MultiprocessingDispatcher {
    pub adapter: Arc<dyn OptimizationAdapter>,
    pub n_jobs: i32,
}

impl MultiprocessingDispatcher {
    pub fn new(adapter: Arc<dyn OptimizationAdapter>) -> Self {
        Self { adapter, n_jobs: 1 }
    }

    pub fn with_n_jobs(adapter: Arc<dyn OptimizationAdapter>, n_jobs: i32) -> Self {
        Self { adapter, n_jobs }
    }
}

impl EvaluationDispatcher for MultiprocessingDispatcher {
    fn dispatch(&self, objective: ObjectiveFn, timer: Option<OptimisationTimer>) -> Evaluator {
        let adapter = self.adapter.clone();
        let n_jobs = self.n_jobs;
        let timer = prepare_timer(timer);
        Arc::new(move |population| {
            evaluate_population_parallel(
                adapter.clone(),
                objective.clone(),
                timer.clone(),
                n_jobs,
                population,
            )
        })
    }
}

#[derive(Clone)]
pub struct SurrogateDispatcher {
    pub adapter: Arc<dyn OptimizationAdapter>,
}

impl Default for SurrogateDispatcher {
    fn default() -> Self {
        Self {
            adapter: Arc::new(DirectAdapter),
        }
    }
}

impl SurrogateDispatcher {
    pub fn new(adapter: Arc<dyn OptimizationAdapter>) -> Self {
        Self { adapter }
    }
}

impl EvaluationDispatcher for SurrogateDispatcher {
    fn dispatch(&self, objective: ObjectiveFn, timer: Option<OptimisationTimer>) -> Evaluator {
        SequentialDispatcher {
            adapter: self.adapter.clone(),
        }
        .dispatch(objective, timer)
    }
}
