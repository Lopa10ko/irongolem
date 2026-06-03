use std::collections::HashMap;
use std::sync::Arc;

use crate::golem::dag::GraphDelegate;
use super::fitness::Fitness;

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
