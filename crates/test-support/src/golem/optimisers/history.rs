use std::sync::Arc;

use crate::golem::dag::GraphDelegate;
use super::fitness::Fitness;

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
