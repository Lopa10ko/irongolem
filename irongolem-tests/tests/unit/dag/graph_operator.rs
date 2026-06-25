use super::fixtures::{
    get_initial_graph, graph, res_graph_test_first, res_graph_test_second, res_graph_test_third,
};

#[test]
fn test_graph_operator_init() {
    // There is no separate operator object, so assert the fixture was built as
    // expected instead.
    let g = graph();
    assert_eq!(g.length(), 5);
    assert_eq!(g.root_nodes().len(), 1);
}

#[test]
fn test_actualise_old_node_children() {
    let mut g = graph();
    let selected_node = g.nodes()[2];
    let new_node = g.add_detached("new_node", &[]);

    g.actualise_old_node_children(selected_node, new_node);
    let updated_parent = g.nodes()[1];

    assert!(g.parents_of(updated_parent).contains(&new_node));
}

#[test]
fn test_sort_nodes() {
    let mut g = graph();
    let selected_node = g.nodes()[2];
    let original_length = g.length();
    let new_node = g.add_detached("new_n1", &[]);
    let new_subroot = g.add_detached("new_n2", &[new_node]);

    g.add_node(new_subroot);
    g.connect_nodes(new_subroot, selected_node);
    g.sort_nodes();

    assert_eq!(g.length(), original_length + 2);
    assert_eq!(g.nodes()[4], new_subroot);
    assert_eq!(g.nodes()[5], new_node);
}

#[test]
fn test_node_children() {
    let g = graph();
    let selected_node = g.nodes()[2];

    let children = g.node_children(selected_node);

    assert_eq!(children.len(), 1);
    assert_eq!(children[0], g.nodes()[1]);
}

#[test]
#[ignore = "graph edit distance (get_distance_between) deferred to a later iteration"]
fn test_distance_to_same_graph_restored() {}

#[test]
#[ignore = "graph edit distance (get_distance_between) deferred to a later iteration"]
fn test_known_distances() {}

#[test]
fn test_disconnect_nodes_method_first() {
    let mut g = get_initial_graph();
    let res_graph = res_graph_test_first();

    let node_e = g.nodes()[4];
    let node_e_root = g.nodes()[0];

    g.disconnect_nodes(node_e, node_e_root, true);

    assert_eq!(res_graph, g);
}

#[test]
fn test_disconnect_nodes_method_second() {
    let mut g = get_initial_graph();
    let res_graph = res_graph_test_second();

    let node_b = g.nodes()[5];
    let node_e = g.nodes()[4];

    g.disconnect_nodes(node_b, node_e, true);

    assert_eq!(res_graph, g);
}

#[test]
fn test_disconnect_nodes_method_third() {
    let mut g = get_initial_graph();
    let res_graph = res_graph_test_third();

    let node_d = g.nodes()[1];
    let root_node_e = g.nodes()[0];

    g.disconnect_nodes(node_d, root_node_e, true);

    assert_eq!(res_graph, g);
}

#[test]
fn test_disconnect_nodes_method_fourth() {
    let graph = get_initial_graph();

    // Try to disconnect nodes between which there is no edge.
    let mut res_graph = graph.clone();
    let node_c = res_graph.nodes()[2];
    let root_node_e = res_graph.nodes()[0];

    res_graph.disconnect_nodes(node_c, root_node_e, true);
    assert_eq!(res_graph, graph);
}

#[test]
fn test_disconnect_nodes_method_fifth() {
    let graph = get_initial_graph();

    // Try to disconnect nodes that are not in this graph.
    let mut res_graph = graph.clone();
    let node_k = res_graph.add_detached("k", &[]);
    let node_m = res_graph.add_detached("m", &[node_k]);

    res_graph.disconnect_nodes(node_k, node_m, true);
    assert_eq!(res_graph, graph);
}

#[test]
fn test_get_edges() {
    let g = graph();

    let l3_n1 = g.nodes()[3];
    let l2_n1 = g.nodes()[2];
    let l2_n2 = g.nodes()[4];
    let l1_n1 = g.nodes()[1];
    let l0_n1 = g.nodes()[0];

    let res_edges = vec![
        (l1_n1, l0_n1),
        (l2_n1, l1_n1),
        (l2_n2, l1_n1),
        (l3_n1, l2_n1),
    ];

    let edges = g.get_edges();
    assert_eq!(res_edges, edges);
}
