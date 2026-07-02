use std::sync::Arc;

use crate::golem::dag::GraphDelegate;

#[derive(Debug, Clone, Default)]
pub struct DirectAdapter;

impl DirectAdapter {
    pub fn adapt(&self, graph: GraphDelegate) -> Arc<GraphDelegate> {
        Arc::new(graph)
    }

    pub fn restore(&self, graph: Arc<GraphDelegate>) -> GraphDelegate {
        Arc::try_unwrap(graph).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[derive(Debug, Default)]
pub struct AdaptRegistry;
