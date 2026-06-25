use irongolem::dag::LinkedGraph;

use super::fixtures::{
    branched_cycled_graph, graph, graph_fifth, graph_first, graph_second, graph_third,
    graph_with_multi_roots_first, joined_branches_graph, simple_cycled_graph,
};

#[test]
fn test_distance_to_primary_level() {
    // node_d -> node_b -> {a, a}; the root is node_d.
    let mut g = LinkedGraph::new();
    let a_first = g.add_detached("a", &[]);
    let a_second = g.add_detached("a", &[]);
    let b = g.add_detached("b", &[a_first, a_second]);
    let d = g.add_detached("d", &[b]);
    g.add_node(d);

    assert_eq!(g.distance_to_primary_level(d), 2);
}

#[test]
fn test_nodes_from_height() {
    let g = graph_first();
    let found_nodes = g.nodes_from_layer(1);
    let true_nodes = g.parents_of(g.root_node().unwrap()).to_vec();
    assert_eq!(true_nodes, found_nodes);
}

#[test]
fn test_distance_to_root_level() {
    let g = graph();
    let selected_node = g.nodes()[2];
    let height = g.distance_to_root_level(selected_node);
    assert_eq!(height, 2);
}

#[test]
fn test_nodes_from_layer() {
    let g = graph();
    let nodes_from_desired_layer = g.nodes_from_layer(2);
    assert_eq!(nodes_from_desired_layer.len(), 2);
}

#[test]
fn test_ordered_subnodes_hierarchy() {
    let mut g = LinkedGraph::new();
    let first_node = g.add_detached("a", &[]);
    let second_node = g.add_detached("b", &[]);
    let third_node = g.add_detached("c", &[first_node, second_node]);
    let root = g.add_detached("d", &[third_node]);
    g.add_node(root);

    let ordered_nodes = g.ordered_subnodes_hierarchy(root).unwrap();

    assert_eq!(ordered_nodes.len(), 4);
    assert_eq!(
        ordered_nodes,
        vec![root, third_node, first_node, second_node]
    );
}

#[test]
fn test_ordered_subnodes_cycle() {
    let mut g = LinkedGraph::new();
    let cycle_node = g.add_detached("knn", &[]);
    let second_node = g.add_detached("knn", &[]);
    let third_node = g.add_detached("lda", &[cycle_node, second_node]);
    let root = g.add_detached("logit", &[third_node]);
    // Close the cycle by making the root a parent of cycle_node.
    g.node_mut(cycle_node).parents = vec![root];

    assert!(g.ordered_subnodes_hierarchy(root).is_err());
}

#[test]
fn test_graph_has_cycle() {
    for cycled in [simple_cycled_graph(), branched_cycled_graph()] {
        assert!(cycled.graph_has_cycle());
    }
    for not_cycled in [graph_first(), graph_second(), graph_third()] {
        assert!(!not_cycled.graph_has_cycle());
    }
}

#[test]
fn test_node_depth() {
    let cases: Vec<(LinkedGraph, Vec<&str>, i64)> = vec![
        (simple_cycled_graph(), vec!["c", "d", "e"], -1),
        (graph_fifth(), vec!["b", "c", "d"], 4),
        (graph_with_multi_roots_first(), vec!["16", "13", "14"], 3),
        (joined_branches_graph(), vec!["d", "f", "c"], 5),
    ];

    for (g, names, correct_depth) in cases {
        let nodes: Vec<_> = names.iter().map(|n| g.nodes_by_name(n)[0]).collect();
        assert_eq!(g.node_depth_of(&nodes), correct_depth);
    }
}
