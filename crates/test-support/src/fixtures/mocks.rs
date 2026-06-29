use std::sync::{Arc, RwLock};

use crate::golem::dag::{GraphDelegate, NodeContent};

#[derive(Debug)]
pub struct MockNode {
    pub content: NodeContent,
    pub nodes_from: Vec<Arc<RwLock<MockNode>>>,
}

impl MockNode {
    pub fn new(name: &str) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            content: NodeContent::new(name),
            nodes_from: Vec::new(),
        }))
    }

    pub fn with_parents(name: &str, parents: Vec<Arc<RwLock<MockNode>>>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            content: NodeContent::new(name),
            nodes_from: parents,
        }))
    }
}

#[derive(Debug, Default)]
pub struct MockDomainStructure {
    pub roots: Vec<Arc<RwLock<MockNode>>>,
}

#[derive(Debug, Default)]
pub struct MockAdapter;

pub fn mock_graph_with_params() -> GraphDelegate {
    GraphDelegate::empty()
}
