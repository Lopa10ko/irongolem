use std::sync::{Arc, RwLock};

use super::linked_graph::{Graph, GraphEdge, LinkedGraph};
use super::linked_graph_node::LinkedGraphNode;
use super::reconnect::ReconnectType;

#[derive(Debug, Clone)]
pub struct GraphDelegate {
    operator: LinkedGraph,
}

impl GraphDelegate {
    pub fn new(root: Arc<RwLock<LinkedGraphNode>>) -> Self {
        Self {
            operator: LinkedGraph::new(root),
        }
    }

    pub fn with_roots(roots: Vec<Arc<RwLock<LinkedGraphNode>>>) -> Self {
        Self {
            operator: LinkedGraph::with_roots(roots),
        }
    }

    pub fn empty() -> Self {
        Self {
            operator: LinkedGraph::default(),
        }
    }

    pub fn operator(&self) -> &LinkedGraph {
        &self.operator
    }

    pub fn operator_mut(&mut self) -> &mut LinkedGraph {
        &mut self.operator
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            operator: self.operator.deep_clone(),
        }
    }
}

impl PartialEq for GraphDelegate {
    fn eq(&self, other: &Self) -> bool {
        self.graphs_equal(other)
    }
}

impl Graph for GraphDelegate {
    fn delete_node(&mut self, node: &Arc<RwLock<LinkedGraphNode>>, reconnect: ReconnectType) {
        self.operator.delete_node(node, reconnect);
    }
    fn delete_subtree(&mut self, node: &Arc<RwLock<LinkedGraphNode>>) {
        self.operator.delete_subtree(node);
    }
    fn update_node(
        &mut self,
        old: &Arc<RwLock<LinkedGraphNode>>,
        new: &Arc<RwLock<LinkedGraphNode>>,
    ) {
        self.operator.update_node(old, new);
    }
    fn update_subtree(
        &mut self,
        old: &Arc<RwLock<LinkedGraphNode>>,
        new: &Arc<RwLock<LinkedGraphNode>>,
    ) {
        self.operator.update_subtree(old, new);
    }
    fn connect_nodes(
        &mut self,
        parent: &Arc<RwLock<LinkedGraphNode>>,
        child: &Arc<RwLock<LinkedGraphNode>>,
    ) {
        self.operator.connect_nodes(parent, child);
    }
    fn disconnect_nodes(
        &mut self,
        parent: &Arc<RwLock<LinkedGraphNode>>,
        child: &Arc<RwLock<LinkedGraphNode>>,
        clean_up_leftovers: bool,
    ) {
        self.operator
            .disconnect_nodes(parent, child, clean_up_leftovers);
    }
    fn add_node(&mut self, node: Arc<RwLock<LinkedGraphNode>>) {
        self.operator.add_node(node);
    }
    fn nodes(&self) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
        self.operator.nodes()
    }
    fn root_node(&self) -> Option<Arc<RwLock<LinkedGraphNode>>> {
        self.operator.root_node()
    }
    fn root_nodes(&self) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
        self.operator.root_nodes()
    }
    fn depth(&self) -> usize {
        self.operator.depth()
    }
    fn length(&self) -> usize {
        self.operator.length()
    }
    fn get_edges(&self) -> Vec<GraphEdge> {
        self.operator.get_edges()
    }
    fn node_children(
        &self,
        node: &Arc<RwLock<LinkedGraphNode>>,
    ) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
        self.operator.node_children(node)
    }
    fn graphs_equal(&self, other: &dyn Graph) -> bool {
        self.operator.graphs_equal(other)
    }
    fn descriptive_id(&self) -> String {
        self.operator.descriptive_id()
    }
}
