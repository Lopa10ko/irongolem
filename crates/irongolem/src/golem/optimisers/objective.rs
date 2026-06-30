use std::collections::HashMap;
use std::sync::Arc;

use super::fitness::Fitness;
use crate::golem::dag::GraphDelegate;

pub struct Objective {
    pub metrics: HashMap<String, String>,
}

impl Objective {
    pub fn new(metrics: HashMap<String, String>) -> Self {
        Self { metrics }
    }

    pub fn metric_names(&self) -> Vec<String> {
        self.metrics.keys().cloned().collect()
    }

    pub fn quality_metrics(&self) -> Vec<String> {
        self.metric_names()
    }

    pub fn complexity_metrics(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn evaluate(&self, _graph: Arc<GraphDelegate>) -> Fitness {
        Fitness::valid_fitness()
    }
}
