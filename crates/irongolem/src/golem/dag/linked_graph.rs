use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use super::convert::{graph_edit_distance, graph_structure_as_digraph};
use super::graph_node::GraphNode;
use super::graph_utils::{graph_has_cycle, ordered_subnodes_hierarchy};
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
    fn disconnect_nodes(
        &mut self,
        parent: &Arc<RwLock<LinkedGraphNode>>,
        child: &Arc<RwLock<LinkedGraphNode>>,
        clean_up_leftovers: bool,
    );
    fn add_node(&mut self, node: Arc<RwLock<LinkedGraphNode>>);
    fn nodes(&self) -> Vec<Arc<RwLock<LinkedGraphNode>>>;
    fn root_node(&self) -> Option<Arc<RwLock<LinkedGraphNode>>>;
    fn root_nodes(&self) -> Vec<Arc<RwLock<LinkedGraphNode>>>;
    fn depth(&self) -> usize;
    fn length(&self) -> usize;
    fn get_edges(&self) -> Vec<GraphEdge>;
    fn node_children(
        &self,
        node: &Arc<RwLock<LinkedGraphNode>>,
    ) -> Vec<Arc<RwLock<LinkedGraphNode>>>;
    fn graphs_equal(&self, other: &dyn Graph) -> bool;
    fn descriptive_id(&self) -> String;
}

#[derive(Debug, Clone, Default)]
pub struct LinkedGraph {
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

    pub fn actualise_old_node_children(
        &mut self,
        old_node: &Arc<RwLock<LinkedGraphNode>>,
        new_node: &Arc<RwLock<LinkedGraphNode>>,
    ) {
        let offspring = self.node_children(old_node);
        for child in offspring {
            let mut child_guard = child.write().unwrap();
            let len = child_guard.nodes_from.len();
            for idx in 0..len {
                if Arc::ptr_eq(&child_guard.nodes_from[idx], old_node) {
                    child_guard.nodes_from[idx] = new_node.clone();
                }
            }
        }
    }

    pub fn sort_nodes(&mut self) {
        if self.root_nodes().len() == 1 && !graph_has_cycle(self) {
            if let Some(root) = self.root_node() {
                if let Ok(sorted) = ordered_subnodes_hierarchy(&root) {
                    self.nodes = sorted;
                }
            }
        }
    }

    fn clean_up_leftovers(&mut self, node: &Arc<RwLock<LinkedGraphNode>>) {
        if self.node_children(node).is_empty() {
            self.nodes.retain(|n| !Arc::ptr_eq(n, node));
            let parents = node.read().unwrap().nodes_from.clone();
            for parent in parents {
                self.clean_up_leftovers(&parent);
            }
        }
    }

    fn clone_node_recursive(
        node: &Arc<RwLock<LinkedGraphNode>>,
        cache: &mut HashMap<usize, Arc<RwLock<LinkedGraphNode>>>,
    ) -> Arc<RwLock<LinkedGraphNode>> {
        let ptr = Arc::as_ptr(node) as usize;
        if let Some(cached) = cache.get(&ptr) {
            return cached.clone();
        }
        let guard = node.read().unwrap();
        let parents: Vec<_> = guard
            .nodes_from
            .iter()
            .map(|p| Self::clone_node_recursive(p, cache))
            .collect();
        let new_node = LinkedGraphNode::with_parents(guard.content.clone(), parents);
        cache.insert(ptr, new_node.clone());
        new_node
    }
}

/// Deep-clones the subtree rooted at `node`.
pub fn clone_subtree(node: &Arc<RwLock<LinkedGraphNode>>) -> Arc<RwLock<LinkedGraphNode>> {
    LinkedGraph::clone_node_recursive(node, &mut HashMap::new())
}

impl LinkedGraph {
    pub fn deep_clone(&self) -> Self {
        let mut cache: HashMap<usize, Arc<RwLock<LinkedGraphNode>>> = HashMap::new();
        for node in &self.nodes {
            Self::clone_node_recursive(node, &mut cache);
        }
        let nodes = self
            .nodes
            .iter()
            .map(|n| cache.get(&(Arc::as_ptr(n) as usize)).unwrap().clone())
            .collect();
        Self { nodes }
    }
}

fn depth_from_node(node: &Arc<RwLock<LinkedGraphNode>>) -> usize {
    depth_from_node_inner(node, &mut HashSet::new()).unwrap_or(0)
}

fn depth_from_node_inner(
    node: &Arc<RwLock<LinkedGraphNode>>,
    visited: &mut HashSet<String>,
) -> Option<usize> {
    let uid = node.read().unwrap().uid.clone();
    if !visited.insert(uid.clone()) {
        return None;
    }
    let parents = node.read().unwrap().nodes_from.clone();
    let result = if parents.is_empty() {
        Some(1)
    } else {
        let mut max_parent = 0;
        for parent in parents {
            let d = depth_from_node_inner(&parent, visited)?;
            max_parent = max_parent.max(d);
        }
        Some(max_parent + 1)
    };
    visited.remove(&uid);
    result
}

impl Graph for LinkedGraph {
    fn delete_node(&mut self, node: &Arc<RwLock<LinkedGraphNode>>, reconnect: ReconnectType) {
        if !self.nodes.iter().any(|n| Arc::ptr_eq(n, node)) {
            return;
        }

        let node_children_cached = self.node_children(node);
        let node_parents = node.read().unwrap().nodes_from.clone();

        self.nodes.retain(|n| !Arc::ptr_eq(n, node));
        for child in &node_children_cached {
            child
                .write()
                .unwrap()
                .nodes_from
                .retain(|p| !Arc::ptr_eq(p, node));
        }

        match reconnect {
            ReconnectType::Single => {
                if !node_parents.is_empty() && node_children_cached.len() == 1 {
                    let child = &node_children_cached[0];
                    child.write().unwrap().nodes_from.extend(node_parents);
                }
            }
            ReconnectType::All => {
                if !node_parents.is_empty() {
                    for child in &node_children_cached {
                        child
                            .write()
                            .unwrap()
                            .nodes_from
                            .extend(node_parents.clone());
                    }
                }
            }
            ReconnectType::None => {}
        }
    }

    fn delete_subtree(&mut self, node: &Arc<RwLock<LinkedGraphNode>>) {
        let subtree_nodes = ordered_subnodes_hierarchy(node).unwrap_or_else(|_| vec![node.clone()]);
        let subtree_ptrs: HashSet<usize> = subtree_nodes
            .iter()
            .map(|n| Arc::as_ptr(n) as usize)
            .collect();
        self.nodes
            .retain(|n| !subtree_ptrs.contains(&(Arc::as_ptr(n) as usize)));
        for n in &self.nodes {
            n.write()
                .unwrap()
                .nodes_from
                .retain(|p| !subtree_ptrs.contains(&(Arc::as_ptr(p) as usize)));
        }
    }

    fn update_node(
        &mut self,
        old: &Arc<RwLock<LinkedGraphNode>>,
        new: &Arc<RwLock<LinkedGraphNode>>,
    ) {
        self.actualise_old_node_children(old, new);
        {
            let old_parents = old.read().unwrap().nodes_from.clone();
            let mut new_guard = new.write().unwrap();
            new_guard.nodes_from.extend(old_parents);
        }
        self.nodes.retain(|n| !Arc::ptr_eq(n, old));
        self.add_node(new.clone());
        self.sort_nodes();
    }

    fn update_subtree(
        &mut self,
        old: &Arc<RwLock<LinkedGraphNode>>,
        new: &Arc<RwLock<LinkedGraphNode>>,
    ) {
        let new_subtree = clone_subtree(new);
        self.actualise_old_node_children(old, &new_subtree);
        self.delete_subtree(old);
        self.add_node(new_subtree);
        self.sort_nodes();
    }
    fn connect_nodes(
        &mut self,
        parent: &Arc<RwLock<LinkedGraphNode>>,
        child: &Arc<RwLock<LinkedGraphNode>>,
    ) {
        if self
            .node_children(parent)
            .iter()
            .any(|c| Arc::ptr_eq(c, child))
        {
            return;
        }
        child.write().unwrap().nodes_from.push(parent.clone());
    }
    fn disconnect_nodes(
        &mut self,
        parent: &Arc<RwLock<LinkedGraphNode>>,
        child: &Arc<RwLock<LinkedGraphNode>>,
        clean_up_leftovers: bool,
    ) {
        let parent_in_child = child
            .read()
            .unwrap()
            .nodes_from
            .iter()
            .any(|p| Arc::ptr_eq(p, parent));
        if !parent_in_child {
            return;
        }
        let parent_in_graph = self.nodes.iter().any(|n| Arc::ptr_eq(n, parent));
        let child_in_graph = self.nodes.iter().any(|n| Arc::ptr_eq(n, child));
        if !parent_in_graph || !child_in_graph {
            return;
        }
        child
            .write()
            .unwrap()
            .nodes_from
            .retain(|p| !Arc::ptr_eq(p, parent));
        if clean_up_leftovers {
            self.clean_up_leftovers(parent);
        }
    }
    fn add_node(&mut self, node: Arc<RwLock<LinkedGraphNode>>) {
        if self.nodes.iter().any(|n| Arc::ptr_eq(n, &node)) {
            return;
        }
        self.nodes.push(node.clone());
        let parents = node.read().unwrap().nodes_from.clone();
        for parent in parents {
            self.add_node(parent);
        }
    }
    fn nodes(&self) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
        self.nodes.clone()
    }
    fn root_node(&self) -> Option<Arc<RwLock<LinkedGraphNode>>> {
        let roots = self.root_nodes();
        if roots.len() == 1 {
            roots.into_iter().next()
        } else {
            roots.first().cloned()
        }
    }
    fn root_nodes(&self) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
        self.nodes
            .iter()
            .filter(|node| self.node_children(node).is_empty())
            .cloned()
            .collect()
    }
    fn depth(&self) -> usize {
        if self.nodes.is_empty() {
            return 0;
        }
        let roots = self.root_nodes();
        if roots.is_empty() {
            return 0;
        }
        roots.iter().map(depth_from_node).max().unwrap_or(0)
    }
    fn length(&self) -> usize {
        self.nodes.len()
    }
    fn get_edges(&self) -> Vec<GraphEdge> {
        let mut edges = Vec::new();
        for node in &self.nodes {
            let parents = node.read().unwrap().nodes_from.clone();
            if !parents.is_empty() {
                for parent in parents {
                    edges.push((parent, node.clone()));
                }
            }
        }
        edges
    }
    fn node_children(
        &self,
        node: &Arc<RwLock<LinkedGraphNode>>,
    ) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
        self.nodes
            .iter()
            .filter(|other| {
                let other = other.read().unwrap();
                !other.nodes_from.is_empty()
                    && other
                        .nodes_from
                        .iter()
                        .any(|parent| Arc::ptr_eq(parent, node))
            })
            .cloned()
            .collect()
    }
    fn graphs_equal(&self, other: &dyn Graph) -> bool {
        let self_roots: HashSet<String> = self
            .root_nodes()
            .iter()
            .map(|n| n.read().unwrap().descriptive_id())
            .collect();
        let other_roots: HashSet<String> = other
            .root_nodes()
            .iter()
            .map(|n| n.read().unwrap().descriptive_id())
            .collect();
        self_roots == other_roots
    }
    fn descriptive_id(&self) -> String {
        if self.length() == 0 {
            return "EMPTY".to_string();
        }
        let roots = self.root_nodes();
        if !roots.is_empty() {
            return roots
                .iter()
                .map(|r| r.read().unwrap().descriptive_id())
                .collect::<Vec<_>>()
                .join("");
        }
        let mut nodes = self.nodes();
        nodes.sort_by(|a, b| a.read().unwrap().uid.cmp(&b.read().unwrap().uid));
        let id = nodes[0].read().unwrap().descriptive_id();
        id
    }
}

pub fn get_distance_between<G: Graph>(a: &G, b: &G) -> i32 {
    if a.graphs_equal(b) {
        return 0;
    }
    let g1 = graph_structure_as_digraph(a);
    let g2 = graph_structure_as_digraph(b);
    graph_edit_distance(&g1, &g2) as i32
}
