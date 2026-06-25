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

    pub fn evaluate(&self, _graph: Arc<GraphDelegate>) -> Fitness {
        Fitness::valid_fitness()
    }
}
