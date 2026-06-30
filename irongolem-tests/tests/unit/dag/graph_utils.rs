//! graph_utils

use std::sync::Arc;

use irongolem::golem::dag::{
    distance_to_primary_level, distance_to_root_level, get_nodes_by_name, graph_has_cycle,
    node_depth, nodes_from_layer, ordered_subnodes_hierarchy, Graph, LinkedGraphNode,
};
use test_support::fixtures::{
    branched_cycled_graph, get_nodes_chain, graph_fifth, graph_first, graph_second, graph_third,
    graph_with_multi_roots_first, joined_branches_graph, operator_test_graph, simple_cycled_graph,
};

fn ptr_eq(
    a: &Arc<std::sync::RwLock<LinkedGraphNode>>,
    b: &Arc<std::sync::RwLock<LinkedGraphNode>>,
) -> bool {
    Arc::ptr_eq(a, b)
}

#[test]
fn test_distance_to_primary_level() {
    let nodes = get_nodes_chain();
    let root = &nodes[0];
    let distance = distance_to_primary_level(root);
    assert_eq!(distance, 2);
}

#[test]
fn test_nodes_from_height() {
    let graph = graph_first();
    let found_nodes = nodes_from_layer(&graph, 1);
    let root = graph.root_node().unwrap();
    let true_nodes = root.read().unwrap().nodes_from.clone();
    assert_eq!(found_nodes.len(), true_nodes.len());
    for (node_model, found_node) in true_nodes.iter().zip(found_nodes.iter()) {
        assert!(ptr_eq(node_model, found_node));
    }
}

#[test]
fn test_distance_to_root_level() {
    let graph = operator_test_graph();
    let selected_node = graph.nodes()[2].clone();
    let height = distance_to_root_level(&graph, &selected_node);
    assert_eq!(height, 2);
}

#[test]
fn test_nodes_from_layer() {
    let graph = operator_test_graph();
    let desired_layer = 2;
    let nodes_from_desired_layer = nodes_from_layer(&graph, desired_layer);
    assert_eq!(nodes_from_desired_layer.len(), 2);
}

#[test]
fn test_ordered_subnodes_hierarchy() {
    let first_node = LinkedGraphNode::from_name("a");
    let second_node = LinkedGraphNode::from_name("b");
    let third_node =
        LinkedGraphNode::with_parents("c", vec![first_node.clone(), second_node.clone()]);
    let root = LinkedGraphNode::with_parents("d", vec![third_node.clone()]);

    let ordered_nodes = ordered_subnodes_hierarchy(&root).unwrap();
    assert_eq!(ordered_nodes.len(), 4);
    assert!(ptr_eq(&ordered_nodes[0], &root));
    assert!(ptr_eq(&ordered_nodes[1], &third_node));
    assert!(ptr_eq(&ordered_nodes[2], &first_node));
    assert!(ptr_eq(&ordered_nodes[3], &second_node));
}

#[test]
fn test_ordered_subnodes_cycle() {
    let cycle_node = LinkedGraphNode::from_name("knn");
    let second_node = LinkedGraphNode::from_name("knn");
    let third_node = LinkedGraphNode::with_parents("lda", vec![cycle_node.clone(), second_node]);
    let root = LinkedGraphNode::with_parents("logit", vec![third_node]);
    cycle_node.write().unwrap().nodes_from.push(root.clone());

    let result = ordered_subnodes_hierarchy(&root);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cycle"));
}

#[test]
fn test_graph_has_cycle() {
    for cycled_graph in [simple_cycled_graph(), branched_cycled_graph()] {
        assert!(graph_has_cycle(&cycled_graph));
    }
    for not_cycled_graph in [graph_first(), graph_second(), graph_third()] {
        assert!(!graph_has_cycle(&not_cycled_graph));
    }
}

#[test]
fn test_graph_has_cycle_diamond_back_edge() {
    // Regression: DFS must not walk parents of a popped stack frame (arena-branch bug).
    let a = LinkedGraphNode::from_name("a");
    let b = LinkedGraphNode::with_parents("b", vec![a.clone()]);
    let c = LinkedGraphNode::with_parents("c", vec![a.clone()]);
    let d = LinkedGraphNode::with_parents("d", vec![b.clone(), c.clone()]);
    b.write().unwrap().nodes_from.push(d.clone());
    let graph = irongolem::golem::dag::GraphDelegate::with_roots(vec![d]);
    assert!(graph_has_cycle(&graph));
}

#[test]
fn test_graph_has_cycle_disconnected_component() {
    let a = LinkedGraphNode::from_name("a");
    let b = LinkedGraphNode::with_parents("b", vec![a.clone()]);
    let x = LinkedGraphNode::from_name("x");
    let y = LinkedGraphNode::with_parents("y", vec![x.clone()]);
    let graph = irongolem::golem::dag::GraphDelegate::with_roots(vec![b, y]);
    assert!(!graph_has_cycle(&graph));
}

#[test]
fn test_node_depth() {
    let cases: Vec<(fn() -> irongolem::golem::dag::GraphDelegate, &[&str], i32)> = vec![
        (simple_cycled_graph, &["c", "d", "e"], -1),
        (graph_fifth, &["b", "c", "d"], 4),
        (graph_with_multi_roots_first, &["16", "13", "14"], 3),
        (joined_branches_graph, &["d", "f", "c"], 5),
    ];

    for (graph_fn, names, expected) in cases {
        let graph = graph_fn();
        let nodes: Vec<_> = names
            .iter()
            .map(|name| get_nodes_by_name(&graph, name)[0].clone())
            .collect();
        let depths = node_depth(&nodes);
        assert_eq!(depths, expected, "failed for graph {:?}", names);
    }
}
