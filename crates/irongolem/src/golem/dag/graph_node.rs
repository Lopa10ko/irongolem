use std::sync::{Arc, RwLock};

use super::linked_graph_node::LinkedGraphNode;

pub trait GraphNode {
    fn description(&self) -> String;
    fn nodes_from(&self) -> &[Arc<RwLock<LinkedGraphNode>>];
    fn descriptive_id(&self) -> String;
}

pub fn descriptive_id(node: &LinkedGraphNode) -> String {
    descriptive_id_recursive(node, &mut Vec::new())
}

/// Returns all nodes in the subtree rooted at `current_node` (pre-order), matching Python
/// `descriptive_id_recursive_nodes`.
pub fn descriptive_id_recursive_nodes(
    current_node: &Arc<RwLock<LinkedGraphNode>>,
    visited_nodes: &mut Vec<Arc<RwLock<LinkedGraphNode>>>,
) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
    if visited_nodes.iter().any(|n| Arc::ptr_eq(n, current_node)) {
        return Vec::new();
    }
    visited_nodes.push(current_node.clone());
    let mut full_path_items = Vec::new();
    let parents = current_node.read().unwrap().nodes_from.clone();
    if !parents.is_empty() {
        for parent_node in parents {
            full_path_items.extend(descriptive_id_recursive_nodes(
                &parent_node,
                &mut visited_nodes.clone(),
            ));
        }
    }
    full_path_items.push(current_node.clone());
    full_path_items
}

fn descriptive_id_recursive(node: &LinkedGraphNode, visited: &mut Vec<String>) -> String {
    let node_label = node.description();
    if visited.contains(&node.uid) {
        return "ID_CYCLED".to_string();
    }
    visited.push(node.uid.clone());

    let mut full_path_items = Vec::new();
    if !node.nodes_from.is_empty() {
        let mut previous_items: Vec<String> = node
            .nodes_from
            .iter()
            .map(|p| {
                format!(
                    "{};",
                    descriptive_id_recursive(&p.read().unwrap(), &mut visited.clone())
                )
            })
            .collect();
        previous_items.sort();
        full_path_items.push(format!("({})", previous_items.join(";")));
    }
    full_path_items.push(format!("/{node_label}"));
    full_path_items.join("")
}
