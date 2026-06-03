use std::sync::Arc;

use crate::golem::adapter::DirectAdapter;
use crate::golem::dag::GraphDelegate;
use super::fitness::Fitness;
use super::history::Individual;
use super::timer::OptimisationTimer;

pub type ObjectiveFn = Arc<dyn Fn(Arc<GraphDelegate>) -> Fitness + Send + Sync>;
pub type Evaluator = Arc<dyn Fn(Vec<Individual>) -> Vec<Individual> + Send + Sync>;

pub trait EvaluationDispatcher {
    fn dispatch(&self, objective: ObjectiveFn, timer: Option<OptimisationTimer>) -> Evaluator;
}

#[derive(Debug, Clone, Default)]
pub struct SequentialDispatcher {
    pub adapter: DirectAdapter,
}

impl SequentialDispatcher {
    pub fn new(adapter: DirectAdapter) -> Self {
        Self { adapter }
    }
}

impl EvaluationDispatcher for SequentialDispatcher {
    fn dispatch(&self, objective: ObjectiveFn, _timer: Option<OptimisationTimer>) -> Evaluator {
        Arc::new(move |population| {
            population
                .into_iter()
                .filter_map(|mut ind| {
                    let f = objective(ind.graph.clone());
                    if f.valid {
                        ind.fitness = f;
                        Some(ind)
                    } else {
                        None
                    }
                })
                .collect()
        })
    }
}

#[derive(Debug, Clone)]
pub struct MultiprocessingDispatcher {
    pub adapter: DirectAdapter,
    pub n_jobs: i32,
}

impl MultiprocessingDispatcher {
    pub fn new(adapter: DirectAdapter) -> Self {
        Self { adapter, n_jobs: 1 }
    }

    pub fn with_n_jobs(adapter: DirectAdapter, n_jobs: i32) -> Self {
        Self { adapter, n_jobs }
    }
}

impl EvaluationDispatcher for MultiprocessingDispatcher {
    fn dispatch(&self, objective: ObjectiveFn, _timer: Option<OptimisationTimer>) -> Evaluator {
        SequentialDispatcher {
            adapter: self.adapter.clone(),
        }
        .dispatch(objective, _timer)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SurrogateDispatcher {
    pub adapter: DirectAdapter,
}

impl SurrogateDispatcher {
    pub fn new(adapter: DirectAdapter) -> Self {
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
