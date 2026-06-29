use std::sync::{Arc, RwLock};

use super::linked_graph_node::LinkedGraphNode;
use super::reconnect::ReconnectType;

pub type GraphEdge = (Arc<RwLock<LinkedGraphNode>>, Arc<RwLock<LinkedGraphNode>>);

pub trait Graph {
    fn delete_node(&mut self, node: &Arc<RwLock<LinkedGraphNode>>, reconnect: ReconnectType);
    fn delete_subtree(&mut self, node: &Arc<RwLock<LinkedGraphNode>>);
    fn update_node(
        &mut self,
        old: &Arc<RwLock<LinkedGraphNode>>,
        new: &Arc<RwLock<LinkedGraphNode>>,
    );
    fn update_subtree(
        &mut self,
        old: &Arc<RwLock<LinkedGraphNode>>,
        new: &Arc<RwLock<LinkedGraphNode>>,
    );
    fn connect_nodes(
        &mut self,
        parent: &Arc<RwLock<LinkedGraphNode>>,
        child: &Arc<RwLock<LinkedGraphNode>>,
    );
    fn add_node(&mut self, node: Arc<RwLock<LinkedGraphNode>>);
    fn nodes(&self) -> Vec<Arc<RwLock<LinkedGraphNode>>>;
    fn root_node(&self) -> Option<Arc<RwLock<LinkedGraphNode>>>;
    fn root_nodes(&self) -> Vec<Arc<RwLock<LinkedGraphNode>>>;
    fn depth(&self) -> usize;
    fn length(&self) -> usize;
    fn get_edges(&self) -> Vec<GraphEdge>;
    fn node_children(&self, node: &Arc<RwLock<LinkedGraphNode>>)
        -> Vec<Arc<RwLock<LinkedGraphNode>>>;
    fn graphs_equal(&self, other: &dyn Graph) -> bool;
    fn descriptive_id(&self) -> String;
}

#[derive(Debug, Clone, Default)]
pub struct LinkedGraph {
    roots: Vec<Arc<RwLock<LinkedGraphNode>>>,
    nodes: Vec<Arc<RwLock<LinkedGraphNode>>>,
}

impl LinkedGraph {
    pub fn new(root: Arc<RwLock<LinkedGraphNode>>) -> Self {
        let mut g = Self::default();
        g.add_node(root);
        g
    }

    pub fn with_roots(roots: Vec<Arc<RwLock<LinkedGraphNode>>>) -> Self {
        let mut g = Self::default();
        for r in roots {
            g.add_node(r);
        }
        g
    }
}

impl Graph for LinkedGraph {
    fn delete_node(&mut self, _node: &Arc<RwLock<LinkedGraphNode>>, _reconnect: ReconnectType) {}
    fn delete_subtree(&mut self, _node: &Arc<RwLock<LinkedGraphNode>>) {}
    fn update_node(&mut self, _old: &Arc<RwLock<LinkedGraphNode>>, _new: &Arc<RwLock<LinkedGraphNode>>) {}
    fn update_subtree(&mut self, _old: &Arc<RwLock<LinkedGraphNode>>, _new: &Arc<RwLock<LinkedGraphNode>>) {}
    fn connect_nodes(&mut self, _parent: &Arc<RwLock<LinkedGraphNode>>, _child: &Arc<RwLock<LinkedGraphNode>>) {}
    fn add_node(&mut self, node: Arc<RwLock<LinkedGraphNode>>) {
        if self.roots.is_empty() {
            self.roots.push(node.clone());
        }
        if !self.nodes.iter().any(|n| Arc::ptr_eq(n, &node)) {
            self.nodes.push(node);
        }
    }
    fn nodes(&self) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
        self.nodes.clone()
    }
    fn root_node(&self) -> Option<Arc<RwLock<LinkedGraphNode>>> {
        self.roots.first().cloned()
    }
    fn root_nodes(&self) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
        self.roots.clone()
    }
    fn depth(&self) -> usize {
        0
    }
    fn length(&self) -> usize {
        self.nodes.len()
    }
    fn get_edges(&self) -> Vec<GraphEdge> {
        Vec::new()
    }
    fn node_children(&self, _node: &Arc<RwLock<LinkedGraphNode>>) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
        Vec::new()
    }
    fn graphs_equal(&self, _other: &dyn Graph) -> bool {
        false
    }
    fn descriptive_id(&self) -> String {
        String::new()
    }
}

pub fn get_distance_between<G: Graph + ?Sized>(_a: &G, _b: &G) -> f64 {
    0.0
}
