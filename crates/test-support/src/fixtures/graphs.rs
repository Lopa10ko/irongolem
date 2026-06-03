use std::sync::{Arc, RwLock};

use crate::golem::dag::{Graph, GraphDelegate, GraphNode, LinkedGraphNode};

#[allow(unused_imports)]
use crate::golem::dag::Graph as _;

pub fn nodes_same(
    left: &[Arc<RwLock<LinkedGraphNode>>],
    right: &[Arc<RwLock<LinkedGraphNode>>],
) -> bool {
    let left_set: std::collections::HashSet<String> = left
        .iter()
        .map(|n| n.read().unwrap().descriptive_id())
        .collect();
    let right_set: std::collections::HashSet<String> = right
        .iter()
        .map(|n| n.read().unwrap().descriptive_id())
        .collect();
    left_set == right_set && left.len() == right.len()
}

pub fn graphs_same(left: &GraphDelegate, right: &GraphDelegate) -> bool {
    left.graphs_equal(right)
}

pub fn find_same_node(
    nodes: &[Arc<RwLock<LinkedGraphNode>>],
    target: &Arc<RwLock<LinkedGraphNode>>,
) -> Option<Arc<RwLock<LinkedGraphNode>>> {
    let target_id = target.read().unwrap().descriptive_id();
    nodes
        .iter()
        .find(|n| n.read().unwrap().descriptive_id() == target_id)
        .cloned()
}

fn link(parent: &Arc<RwLock<LinkedGraphNode>>, child: Arc<RwLock<LinkedGraphNode>>) {
    parent.write().unwrap().nodes_from.push(child);
}

pub fn graph_first() -> GraphDelegate {
    let root_of_tree = LinkedGraphNode::from_name("a");
    let root_child_first = LinkedGraphNode::from_name("a");
    let root_child_second = LinkedGraphNode::from_name("b");

    for root_node_child in [&root_child_first, &root_child_second] {
        for oper in ["c", "d"] {
            let new_node = LinkedGraphNode::from_name(oper);
            link(root_node_child, new_node);
        }
        link(&root_of_tree, root_node_child.clone());
    }

    GraphDelegate::new(root_of_tree)
}

pub fn graph_second() -> GraphDelegate {
    let new_node = LinkedGraphNode::from_name("a");
    for oper in ["b", "d"] {
        link(&new_node, LinkedGraphNode::from_name(oper));
    }
    let graph = graph_first();
    let _ = new_node;
    graph
}

pub fn graph_third() -> GraphDelegate {
    let root_of_tree = LinkedGraphNode::from_name("a");
    for oper in ["b", "d", "b"] {
        link(&root_of_tree, LinkedGraphNode::from_name(oper));
    }
    GraphDelegate::new(root_of_tree)
}

pub fn graph_fourth() -> GraphDelegate {
    let graph = graph_third();
    let new_node = LinkedGraphNode::from_name("a");
    for _ in 0..2 {
        link(&new_node, LinkedGraphNode::from_name("b"));
    }
    let _ = new_node;
    graph
}

pub fn graph_fifth() -> GraphDelegate {
    let node_a_primary = LinkedGraphNode::from_name("a");
    let node_f = LinkedGraphNode::with_parents("f", vec![node_a_primary.clone()]);
    let node_b_primary = LinkedGraphNode::from_name("b");
    let node_c = LinkedGraphNode::with_parents("c", vec![node_f, node_b_primary]);
    let node_d = LinkedGraphNode::with_parents("d", vec![node_c]);
    let node_e = LinkedGraphNode::with_parents("e", vec![node_d]);
    GraphDelegate::new(node_e)
}

pub fn graph_sixth() -> GraphDelegate {
    let node_a = LinkedGraphNode::from_name("a");
    let node_b = LinkedGraphNode::with_parents("b", vec![node_a]);
    let node_c = LinkedGraphNode::with_parents("c", vec![node_b]);
    GraphDelegate::new(node_c)
}

pub fn graph_seventh() -> GraphDelegate {
    let node_a = LinkedGraphNode::from_name("a");
    let node_b = LinkedGraphNode::with_parents("b", vec![node_a.clone()]);
    let node_c = LinkedGraphNode::with_parents("c", vec![node_a]);
    let mut g = GraphDelegate::empty();
    g.add_node(node_b);
    g.add_node(node_c);
    g
}

pub fn graph_eighth() -> GraphDelegate {
    let node_a = LinkedGraphNode::from_name("a");
    let node_b = LinkedGraphNode::from_name("b");
    let node_c = LinkedGraphNode::with_parents("c", vec![node_a, node_b]);
    GraphDelegate::new(node_c)
}

pub fn graph_ninth() -> GraphDelegate {
    let node_a = LinkedGraphNode::from_name("a");
    let node_b = LinkedGraphNode::with_parents("b", vec![node_a]);
    let node_c = LinkedGraphNode::from_name("c");
    let mut g = GraphDelegate::empty();
    g.add_node(node_b);
    g.add_node(node_c);
    g
}

pub fn graph_with_multi_roots_first() -> GraphDelegate {
    let node1 = LinkedGraphNode::from_name("11");
    let node2 = LinkedGraphNode::from_name("12");
    let node3 = LinkedGraphNode::from_name("13");
    let node4 = LinkedGraphNode::with_parents("14", vec![node1, node2]);
    let node5 = LinkedGraphNode::with_parents("15", vec![node3]);
    let node6 = LinkedGraphNode::with_parents("16", vec![node4, node5.clone()]);
    let node7 = LinkedGraphNode::with_parents("17", vec![node5]);
    GraphDelegate::with_roots(vec![node6, node7])
}

pub fn graph_with_multi_roots_second() -> GraphDelegate {
    let node21 = LinkedGraphNode::from_name("21");
    let node22 = LinkedGraphNode::from_name("22");
    let node23 = LinkedGraphNode::with_parents("23", vec![node21, node22.clone()]);
    let node24 = LinkedGraphNode::with_parents("24", vec![node22]);
    GraphDelegate::with_roots(vec![node23, node24])
}

pub fn graph_with_single_node() -> GraphDelegate {
    GraphDelegate::new(LinkedGraphNode::from_name("a"))
}

pub fn simple_linear_graph() -> GraphDelegate {
    let node_a_primary = LinkedGraphNode::from_name("a");
    let node_b = LinkedGraphNode::with_parents("b", vec![node_a_primary]);
    let node_c = LinkedGraphNode::with_parents("c", vec![node_b]);
    GraphDelegate::new(node_c)
}

pub fn tree_graph() -> GraphDelegate {
    let node_a_primary = LinkedGraphNode::from_name("a");
    let node_b_primary = LinkedGraphNode::from_name("b");
    let node_c = LinkedGraphNode::with_parents("c", vec![node_a_primary, node_b_primary]);
    let node_d = LinkedGraphNode::with_parents("d", vec![node_c]);
    GraphDelegate::new(node_d)
}

pub fn simple_cycled_graph() -> GraphDelegate {
    let node_a_primary = LinkedGraphNode::from_name("a");
    let node_b = LinkedGraphNode::with_parents("b", vec![node_a_primary]);
    let node_c = LinkedGraphNode::with_parents("c", vec![node_b.clone()]);
    let node_d = LinkedGraphNode::with_parents("d", vec![node_c]);
    let node_e = LinkedGraphNode::with_parents("e", vec![node_d.clone()]);
    node_b.write().unwrap().nodes_from.push(node_e);
    GraphDelegate::new(node_d)
}

pub fn branched_cycled_graph() -> GraphDelegate {
    let node_a_primary = LinkedGraphNode::from_name("a");
    let node_b = LinkedGraphNode::with_parents("b", vec![node_a_primary.clone()]);
    let node_c = LinkedGraphNode::with_parents("c", vec![node_b.clone()]);
    let node_d = LinkedGraphNode::with_parents("d", vec![node_c]);
    let node_e = LinkedGraphNode::with_parents("e", vec![node_d.clone()]);
    node_b.write().unwrap().nodes_from.push(node_e);

    let node_f = LinkedGraphNode::with_parents("f", vec![node_a_primary]);
    let node_g = LinkedGraphNode::with_parents("g", vec![node_f.clone()]);
    let node_h = LinkedGraphNode::with_parents("h", vec![node_f]);
    GraphDelegate::with_roots(vec![node_d, node_g, node_h])
}

pub fn joined_branches_graph() -> GraphDelegate {
    let node_a = LinkedGraphNode::from_name("a");
    let node_b = LinkedGraphNode::with_parents("b", vec![node_a.clone()]);
    let node_c = LinkedGraphNode::with_parents("c", vec![node_b.clone(), node_a]);
    let node_d = LinkedGraphNode::with_parents("d", vec![node_c]);
    let node_f = LinkedGraphNode::with_parents("f", vec![node_d, node_b]);
    GraphDelegate::new(node_f)
}

/// Equality cases for graph comparison tests (ported from pytest fixture).
pub fn equality_cases() -> Vec<(GraphDelegate, GraphDelegate)> {
    vec![
        (graph_first(), graph_first()),
        (graph_third(), graph_third()),
        (graph_fourth(), graph_fourth()),
    ]
}

/// Non-equality pairs: first×second, first×third, second×third.
pub fn non_equality_cases() -> Vec<(GraphDelegate, GraphDelegate)> {
    vec![
        (graph_first(), graph_second()),
        (graph_first(), graph_third()),
        (graph_second(), graph_third()),
    ]
}
