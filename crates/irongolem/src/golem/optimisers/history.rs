use std::fmt;
use std::sync::Arc;

use uuid::Uuid;

use super::fitness::Fitness;
use crate::golem::dag::GraphDelegate;

#[derive(Debug, Clone)]
pub struct Individual {
    pub uid: String,
    pub graph: Arc<GraphDelegate>,
    pub fitness: Fitness,
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Eq for Individual {}

impl fmt::Display for Individual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uid)
    }
}

impl Individual {
    pub fn new(graph: Arc<GraphDelegate>) -> Self {
        Self::with_uid(Uuid::new_v4().to_string(), graph)
    }

    pub fn with_uid(uid: impl Into<String>, graph: Arc<GraphDelegate>) -> Self {
        Self {
            uid: uid.into(),
            graph,
            fitness: Fitness::default(),
        }
    }

    pub fn with_fitness(graph: Arc<GraphDelegate>, fitness: Fitness) -> Self {
        Self {
            uid: Uuid::new_v4().to_string(),
            graph,
            fitness,
        }
    }

    pub fn set_fitness(&mut self, fitness: Fitness) {
        self.fitness = fitness;
    }

    pub fn with_graph(mut self, graph: Arc<GraphDelegate>) -> Self {
        self.graph = graph;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParetoFront {
    pub items: Vec<Individual>,
    pub maxsize: usize,
}

impl ParetoFront {
    pub fn new(maxsize: usize) -> Self {
        Self {
            items: Vec::new(),
            maxsize,
        }
    }

    pub fn update(&mut self, new_items: Vec<Individual>) {
        self.items.extend(new_items);
        if self.items.len() > self.maxsize {
            self.items.truncate(self.maxsize);
        }
    }
}
