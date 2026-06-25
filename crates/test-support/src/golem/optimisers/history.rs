use std::sync::Arc;

use super::fitness::Fitness;
use crate::golem::dag::GraphDelegate;

#[derive(Debug, Clone)]
pub struct Individual {
    pub graph: Arc<GraphDelegate>,
    pub fitness: Fitness,
}

impl Individual {
    pub fn new(graph: Arc<GraphDelegate>) -> Self {
        Self {
            graph,
            fitness: Fitness::default(),
        }
    }
}
