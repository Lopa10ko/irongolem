use std::sync::{Arc, RwLock};

use crate::golem::dag::{Graph, GraphDelegate, LinkedGraphNode};

type NodeArc = Arc<RwLock<LinkedGraphNode>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoveType {
    WithDirectChildren,
    WithParents,
    NodeRewire,
    NodeOnly,
    Forbidden,
}

pub trait GraphAdvisor: Send + Sync {
    fn can_be_removed(&self, node: &NodeArc, graph: &GraphDelegate) -> RemoveType;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultAdvisor;

impl GraphAdvisor for DefaultAdvisor {
    fn can_be_removed(&self, node: &NodeArc, graph: &GraphDelegate) -> RemoveType {
        if graph.length() < 2 {
            return RemoveType::Forbidden;
        }
        let children = graph.node_children(node);
        if children.is_empty() {
            RemoveType::WithParents
        } else {
            RemoveType::NodeRewire
        }
    }
}
