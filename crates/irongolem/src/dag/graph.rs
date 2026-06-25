use std::collections::{BTreeSet, HashMap, HashSet};

use super::node::{GraphNode, NodeContent, NodeId};
use super::reconnect::ReconnectType;

/// Raised when an operation requires an acyclic graph but a cycle is present.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CycleError;

impl std::fmt::Display for CycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "graph has cycle")
    }
}

impl std::error::Error for CycleError {}

/// Directed graph backed by an arena.
///
/// `arena` owns every node ever created in the graph and hands out stable
/// [`NodeId`] indices; `order` is the active node list kept in insertion/sorted
/// order. The structure holds only plain data with no pointers or locks, so it is
/// cheap to `Clone` and is `Send + Sync`.
#[derive(Clone, Debug, Default)]
pub struct LinkedGraph {
    arena: Vec<GraphNode>,
    order: Vec<NodeId>,
}

impl LinkedGraph {
    /// Returns an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    // --- arena helpers -----------------------------------------------------

    /// Creates a node in the arena and returns its id without adding it to the
    /// active node set.
    ///
    /// Duplicate `parents` are dropped, so the resulting node holds each parent at
    /// most once.
    pub fn add_detached(&mut self, content: impl Into<NodeContent>, parents: &[NodeId]) -> NodeId {
        let mut unique = Vec::with_capacity(parents.len());
        for &p in parents {
            if !unique.contains(&p) {
                unique.push(p);
            }
        }
        let id = NodeId(self.arena.len());
        self.arena.push(GraphNode::new(content.into(), unique));
        id
    }

    /// Returns a shared reference to the node with the given id.
    pub fn node(&self, id: NodeId) -> &GraphNode {
        &self.arena[id.0]
    }

    /// Returns a mutable reference to the node with the given id.
    pub fn node_mut(&mut self, id: NodeId) -> &mut GraphNode {
        &mut self.arena[id.0]
    }

    /// Returns the parents of the node with the given id.
    pub fn parents_of(&self, id: NodeId) -> &[NodeId] {
        &self.arena[id.0].parents
    }

    /// Returns the active nodes in order.
    pub fn nodes(&self) -> Vec<NodeId> {
        self.order.clone()
    }

    /// Returns the active nodes in order as a slice.
    pub fn order(&self) -> &[NodeId] {
        &self.order
    }

    /// Returns the number of active nodes.
    pub fn length(&self) -> usize {
        self.order.len()
    }

    /// Returns `true` when the graph has no active nodes.
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    /// Returns the description label of the node with the given id.
    pub fn description(&self, id: NodeId) -> String {
        self.arena[id.0].description()
    }

    /// Returns every active node whose operation name equals `name`.
    pub fn nodes_by_name(&self, name: &str) -> Vec<NodeId> {
        self.order
            .iter()
            .copied()
            .filter(|&id| self.arena[id.0].name() == name)
            .collect()
    }

    fn extend_parents(&mut self, child: NodeId, extra: &[NodeId]) {
        for &p in extra {
            if !self.arena[child.0].parents.contains(&p) {
                self.arena[child.0].parents.push(p);
            }
        }
    }

    // --- structure ---------------------------------------------------------

    /// Adds `id` and, recursively, its parents to the active node set in
    /// pre-order.
    pub fn add_node(&mut self, id: NodeId) {
        if self.order.contains(&id) {
            return;
        }
        self.order.push(id);
        let parents = self.arena[id.0].parents.clone();
        for p in parents {
            self.add_node(p);
        }
    }

    /// Returns the children of `node`: active nodes that list `node` as a parent.
    pub fn node_children(&self, node: NodeId) -> Vec<NodeId> {
        self.order
            .iter()
            .copied()
            .filter(|&other| self.arena[other.0].parents.contains(&node))
            .collect()
    }

    /// Returns the active nodes with no children, i.e. the outputs of the graph.
    pub fn root_nodes(&self) -> Vec<NodeId> {
        self.order
            .iter()
            .copied()
            .filter(|&id| self.node_children(id).is_empty())
            .collect()
    }

    /// Returns the first root node, if the graph has any.
    pub fn root_node(&self) -> Option<NodeId> {
        self.root_nodes().into_iter().next()
    }

    /// Returns every edge as a `(parent, child)` pair, in node order.
    pub fn get_edges(&self) -> Vec<(NodeId, NodeId)> {
        let mut edges = Vec::new();
        for &node in &self.order {
            for &parent in &self.arena[node.0].parents {
                edges.push((parent, node));
            }
        }
        edges
    }

    /// Adds an edge `parent -> child` by registering `parent` as a parent of
    /// `child`.
    pub fn connect_nodes(&mut self, parent: NodeId, child: NodeId) {
        if self.node_children(parent).contains(&child) {
            return;
        }
        if !self.arena[child.0].parents.contains(&parent) {
            self.arena[child.0].parents.push(parent);
        }
    }

    /// Removes the edge `parent -> child`, optionally pruning nodes that become
    /// disconnected as a result.
    pub fn disconnect_nodes(&mut self, parent: NodeId, child: NodeId, clean_up_leftovers: bool) {
        if !self.arena[child.0].parents.contains(&parent) {
            return;
        }
        if !self.order.contains(&parent) || !self.order.contains(&child) {
            return;
        }
        self.arena[child.0].parents.retain(|&p| p != parent);
        if clean_up_leftovers {
            self.clean_up_leftovers(parent);
        }
    }

    fn clean_up_leftovers(&mut self, node: NodeId) {
        if self.node_children(node).is_empty() {
            self.order.retain(|&n| n != node);
            let parents = self.arena[node.0].parents.clone();
            for p in parents {
                self.clean_up_leftovers(p);
            }
        }
    }

    /// Removes `node`, reconnecting edges according to `reconnect`.
    pub fn delete_node(&mut self, node: NodeId, reconnect: ReconnectType) {
        let children = self.node_children(node);
        self.order.retain(|&n| n != node);
        for &child in &children {
            self.arena[child.0].parents.retain(|&p| p != node);
        }
        let node_parents = self.arena[node.0].parents.clone();
        match reconnect {
            ReconnectType::Single => {
                if !node_parents.is_empty() && children.len() == 1 {
                    self.extend_parents(children[0], &node_parents);
                }
            }
            ReconnectType::All => {
                if !node_parents.is_empty() {
                    for &child in &children {
                        self.extend_parents(child, &node_parents);
                    }
                }
            }
            ReconnectType::None => {}
        }
    }

    /// Removes `node` together with all of its ancestors, pruning dangling edges.
    pub fn delete_subtree(&mut self, node: NodeId) {
        let subtree: HashSet<NodeId> = self
            .ordered_subnodes_hierarchy(node)
            .unwrap_or_default()
            .into_iter()
            .collect();
        self.order.retain(|n| !subtree.contains(n));
        let active = self.order.clone();
        for n in active {
            self.arena[n.0].parents.retain(|p| !subtree.contains(p));
        }
    }

    /// Repoints the children of `old_node` so that they reference `new_node`.
    pub fn actualise_old_node_children(&mut self, old_node: NodeId, new_node: NodeId) {
        let children = self.node_children(old_node);
        for child in children {
            if let Some(idx) = self.arena[child.0]
                .parents
                .iter()
                .position(|&p| p == old_node)
            {
                self.arena[child.0].parents[idx] = new_node;
            }
        }
    }

    /// Replaces `old_node` with `new_node`, inheriting `old_node`'s parents.
    pub fn update_node(&mut self, old_node: NodeId, new_node: NodeId) {
        self.actualise_old_node_children(old_node, new_node);
        let old_parents = self.arena[old_node.0].parents.clone();
        self.extend_parents(new_node, &old_parents);
        self.order.retain(|&n| n != old_node);
        self.add_node(new_node);
        self.sort_nodes();
    }

    /// Replaces the subtree rooted at `old_subtree` with `new_subtree`.
    pub fn update_subtree(&mut self, old_subtree: NodeId, new_subtree: NodeId) {
        self.actualise_old_node_children(old_subtree, new_subtree);
        self.delete_subtree(old_subtree);
        self.add_node(new_subtree);
        self.sort_nodes();
    }

    /// Re-sorts the active node set into pre-order from the single root.
    ///
    /// The order is only updated when the graph is single-rooted and acyclic;
    /// otherwise it is left unchanged.
    pub fn sort_nodes(&mut self) {
        let roots = self.root_nodes();
        if roots.len() == 1 && !self.graph_has_cycle() {
            if let Ok(order) = self.ordered_subnodes_hierarchy(roots[0]) {
                self.order = order;
            }
        }
    }

    // --- identifiers & equality -------------------------------------------

    /// Returns the recursive structural id of the subgraph ending at `node`.
    pub fn node_descriptive_id(&self, node: NodeId) -> String {
        self.node_descriptive_id_rec(node, &mut Vec::new())
    }

    fn node_descriptive_id_rec(&self, node: NodeId, visited: &mut Vec<NodeId>) -> String {
        let label = self.arena[node.0].description();
        if visited.contains(&node) {
            return "ID_CYCLED".to_string();
        }
        visited.push(node);

        let mut full = String::new();
        let parents = &self.arena[node.0].parents;
        if !parents.is_empty() {
            let mut items: Vec<String> = parents
                .iter()
                .map(|&p| {
                    let mut branch = visited.clone();
                    format!("{};", self.node_descriptive_id_rec(p, &mut branch))
                })
                .collect();
            items.sort();
            full.push('(');
            full.push_str(&items.join(";"));
            full.push(')');
        }
        full.push('/');
        full.push_str(&label);
        full
    }

    /// Returns the structural id of the whole graph.
    pub fn descriptive_id(&self) -> String {
        if self.order.is_empty() {
            return "EMPTY".to_string();
        }
        let roots = self.root_nodes();
        if !roots.is_empty() {
            roots
                .iter()
                .map(|&r| self.node_descriptive_id(r))
                .collect::<String>()
        } else {
            let min = self
                .order
                .iter()
                .copied()
                .min_by_key(|&id| self.arena[id.0].uid)
                .expect("non-empty");
            self.node_descriptive_id(min)
        }
    }

    fn root_descriptive_ids(&self) -> BTreeSet<String> {
        self.root_nodes()
            .iter()
            .map(|&r| self.node_descriptive_id(r))
            .collect()
    }

    // --- graph_utils (operate on the active node set) ----------------------

    /// Pre-order subnode hierarchy starting at `node`; errors on a cycle.
    pub fn ordered_subnodes_hierarchy(&self, node: NodeId) -> Result<Vec<NodeId>, CycleError> {
        let mut started = HashSet::new();
        let mut visited = HashSet::new();
        started.insert(node);
        self.subtree_impl(node, &mut started, &mut visited)
    }

    fn subtree_impl(
        &self,
        node: NodeId,
        started: &mut HashSet<NodeId>,
        visited: &mut HashSet<NodeId>,
    ) -> Result<Vec<NodeId>, CycleError> {
        let mut nodes = vec![node];
        let parents = self.arena[node.0].parents.clone();
        for parent in parents {
            if visited.contains(&parent) {
                continue;
            }
            if started.contains(&parent) {
                return Err(CycleError);
            }
            started.insert(parent);
            nodes.extend(self.subtree_impl(parent, started, visited)?);
            visited.insert(parent);
        }
        Ok(nodes)
    }

    /// True if the graph contains a cycle (DFS over the active node set).
    pub fn graph_has_cycle(&self) -> bool {
        let mut visited: HashMap<NodeId, bool> = self.order.iter().map(|&n| (n, false)).collect();
        let mut on_stack: HashMap<NodeId, bool> = self.order.iter().map(|&n| (n, false)).collect();

        for &start in &self.order {
            if visited[&start] {
                continue;
            }
            let mut stack = vec![start];
            while let Some(&cur) = stack.last() {
                if !visited[&cur] {
                    visited.insert(cur, true);
                    on_stack.insert(cur, true);
                } else {
                    on_stack.insert(cur, false);
                    stack.pop();
                }
                for &parent in &self.arena[cur.0].parents {
                    if !visited.get(&parent).copied().unwrap_or(true) {
                        stack.push(parent);
                    } else if on_stack.get(&parent).copied().unwrap_or(false) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Maximal depth (in node count) among `nodes`; `-1` if a cycle is reachable.
    pub fn node_depth_of(&self, nodes: &[NodeId]) -> i64 {
        let mut memo: HashMap<NodeId, i64> = HashMap::new();
        let mut best = 0i64;
        for &n in nodes {
            match self.longest_path(n, &mut Vec::new(), &mut memo) {
                None => return -1,
                Some(d) => best = best.max(d),
            }
        }
        best
    }

    fn longest_path(
        &self,
        node: NodeId,
        on_path: &mut Vec<NodeId>,
        memo: &mut HashMap<NodeId, i64>,
    ) -> Option<i64> {
        if on_path.contains(&node) {
            return None;
        }
        if let Some(&d) = memo.get(&node) {
            return Some(d);
        }
        on_path.push(node);
        let mut best = 1i64;
        let parents = self.arena[node.0].parents.clone();
        for parent in parents {
            match self.longest_path(parent, on_path, memo) {
                None => {
                    on_path.pop();
                    return None;
                }
                Some(d) => best = best.max(d + 1),
            }
        }
        on_path.pop();
        memo.insert(node, best);
        Some(best)
    }

    /// Distance from `node` up to the primary (input) level.
    pub fn distance_to_primary_level(&self, node: NodeId) -> i64 {
        let depth = self.node_depth_of(&[node]);
        if depth > 0 {
            depth - 1
        } else {
            -1
        }
    }

    /// Distance from `node` down to the root (output) level; `-1` on a cycle.
    pub fn distance_to_root_level(&self, node: NodeId) -> i64 {
        if self.graph_has_cycle() {
            return -1;
        }
        let mut height = 0i64;
        let mut current = node;
        for _ in 0..self.order.len() {
            let children = self.node_children(current);
            if let Some(&first) = children.first() {
                height += 1;
                current = first;
            } else {
                return height;
            }
        }
        height
    }

    /// All nodes exactly `layer` steps above the roots (with multiplicity).
    pub fn nodes_from_layer(&self, layer: i64) -> Vec<NodeId> {
        let mut out = Vec::new();
        let roots = self.root_nodes();
        self.collect_layer(&roots, 0, layer, &mut out);
        out
    }

    fn collect_layer(&self, roots: &[NodeId], current: i64, layer: i64, out: &mut Vec<NodeId>) {
        if current == layer {
            out.extend_from_slice(roots);
        } else {
            for &root in roots {
                let parents = self.arena[root.0].parents.clone();
                self.collect_layer(&parents, current + 1, layer, out);
            }
        }
    }

    /// Maximal depth of the graph; `0` if empty, `-1` if it has no root or a cycle.
    pub fn depth(&self) -> i64 {
        if self.order.is_empty() {
            return 0;
        }
        let roots = self.root_nodes();
        if roots.is_empty() || self.graph_has_cycle() {
            return -1;
        }
        self.node_depth_of(&roots)
    }
}

impl PartialEq for LinkedGraph {
    fn eq(&self, other: &Self) -> bool {
        self.root_descriptive_ids() == other.root_descriptive_ids()
    }
}
