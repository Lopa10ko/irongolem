//! graph_operator

use std::sync::Arc;

use irongolem::golem::dag::{Graph, LinkedGraph, LinkedGraphNode};
use test_support::fixtures::{
    get_initial_graph, get_res_graph_test_first, get_res_graph_test_second,
    get_res_graph_test_third, operator_test_graph,
};

fn edges_equal(
    left: &[(
        Arc<std::sync::RwLock<LinkedGraphNode>>,
        Arc<std::sync::RwLock<LinkedGraphNode>>,
    )],
    right: &[(
        Arc<std::sync::RwLock<LinkedGraphNode>>,
        Arc<std::sync::RwLock<LinkedGraphNode>>,
    )],
) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right.iter())
            .all(|((lp, lc), (rp, rc))| Arc::ptr_eq(lp, rp) && Arc::ptr_eq(lc, rc))
}

fn parent_contains(
    node: &Arc<std::sync::RwLock<LinkedGraphNode>>,
    parent: &Arc<std::sync::RwLock<LinkedGraphNode>>,
) -> bool {
    node.read()
        .unwrap()
        .nodes_from
        .iter()
        .any(|p| Arc::ptr_eq(p, parent))
}

#[test]
fn test_get_edges() {
    let graph = operator_test_graph();
    let nodes = graph.nodes();

    let l3_n1 = nodes[3].clone();
    let l2_n1 = nodes[2].clone();
    let l2_n2 = nodes[4].clone();
    let l1_n1 = nodes[1].clone();
    let l0_n1 = nodes[0].clone();

    let res_edges = vec![
        (l1_n1.clone(), l0_n1.clone()),
        (l2_n1.clone(), l1_n1.clone()),
        (l2_n2.clone(), l1_n1.clone()),
        (l3_n1.clone(), l2_n1.clone()),
    ];

    let edges = graph.get_edges();
    assert!(edges_equal(&res_edges, &edges));
}

#[test]
fn test_graph_operator_init() {
    let graph = operator_test_graph();
    let _: &LinkedGraph = graph.operator();
}

#[test]
fn test_actualise_old_node_children() {
    let mut graph = operator_test_graph();
    let selected_node = graph.nodes()[2].clone();
    let new_node = LinkedGraphNode::from_name("new_node");

    graph
        .operator_mut()
        .actualise_old_node_children(&selected_node, &new_node);

    let updated_parent = graph.nodes()[1].clone();
    assert!(parent_contains(&updated_parent, &new_node));
}

#[test]
fn test_sort_nodes() {
    let mut graph = operator_test_graph();
    let selected_node = graph.nodes()[2].clone();
    let original_length = graph.length();
    let new_node = LinkedGraphNode::from_name("new_n1");
    let new_subroot = LinkedGraphNode::with_parents("new_n2", vec![new_node.clone()]);

    graph.add_node(new_subroot.clone());
    graph.connect_nodes(&new_subroot, &selected_node);
    graph.operator_mut().sort_nodes();

    assert_eq!(graph.length(), original_length + 2);
    assert!(Arc::ptr_eq(&graph.nodes()[4], &new_subroot));
    assert!(Arc::ptr_eq(&graph.nodes()[5], &new_node));
}

#[test]
fn test_node_children() {
    let graph = operator_test_graph();
    let selected_node = graph.nodes()[2].clone();
    let children = graph.node_children(&selected_node);
    assert_eq!(children.len(), 1);
    assert!(Arc::ptr_eq(&children[0], &graph.nodes()[1]));
}

#[test]
fn test_distance_to_same_graph_restored() {
    use irongolem::golem::adapter::DirectAdapter;
    use irongolem::golem::dag::get_distance_between;

    let graph = operator_test_graph();
    let adapter = DirectAdapter::default();
    let opt_graph = adapter.adapt(graph.clone());
    let distance = get_distance_between(&graph, &adapter.restore(opt_graph));
    assert_eq!(distance, 0);
}

#[test]
fn test_known_distances() {
    use std::collections::HashMap;

    use irongolem::golem::dag::{
        get_distance_between, GraphDelegate, LinkedGraphNode, NodeContent,
    };
    use serde_json::json;

    let node_a = LinkedGraphNode::from_name("a");
    let node_b = LinkedGraphNode::from_name("b");
    let node_c = LinkedGraphNode::with_parents("c", vec![node_a.clone()]);
    let mut params = HashMap::new();
    params.insert("alpha".to_string(), json!(4));
    let node_c_alt = LinkedGraphNode::new(NodeContent::with_params("c", params));
    node_c_alt.write().unwrap().nodes_from.push(node_a.clone());
    let node_d = LinkedGraphNode::with_parents("d", vec![node_a.clone()]);

    let graph_a = GraphDelegate::new(node_a);
    let graph_b = GraphDelegate::new(node_b);
    let graph_c = GraphDelegate::new(node_c);
    let graph_d = GraphDelegate::new(node_d);
    let graph_c_alt = GraphDelegate::new(node_c_alt);

    assert_eq!(get_distance_between(&graph_c, &graph_c), 0);
    assert_eq!(get_distance_between(&graph_c, &graph_a), 2);
    assert_eq!(get_distance_between(&graph_c, &graph_d), 1);
    assert_eq!(get_distance_between(&graph_c, &graph_c_alt), 1);
    assert_eq!(get_distance_between(&graph_c, &graph_b), 3);
}

#[test]
fn test_disconnect_nodes_method_first() {
    let mut graph = get_initial_graph();
    let res_graph = get_res_graph_test_first();

    let node_e = graph.nodes()[4].clone();
    let node_e_root = graph.nodes()[0].clone();

    graph.disconnect_nodes(&node_e, &node_e_root, true);
    assert_eq!(res_graph, graph);
}

#[test]
fn test_disconnect_nodes_method_second() {
    let mut graph = get_initial_graph();
    let res_graph = get_res_graph_test_second();

    let node_b = graph.nodes()[5].clone();
    let node_e = graph.nodes()[4].clone();

    graph.disconnect_nodes(&node_b, &node_e, true);
    assert_eq!(res_graph, graph);
}

#[test]
fn test_disconnect_nodes_method_third() {
    let mut graph = get_initial_graph();
    let res_graph = get_res_graph_test_third();

    let node_d = graph.nodes()[1].clone();
    let root_node_e = graph.nodes()[0].clone();

    graph.disconnect_nodes(&node_d, &root_node_e, true);
    assert_eq!(res_graph, graph);
}

#[test]
fn test_disconnect_nodes_method_fourth() {
    let graph = get_initial_graph();
    let mut res_graph = graph.clone();

    let node_c = res_graph.nodes()[2].clone();
    let root_node_e = res_graph.nodes()[0].clone();

    res_graph.disconnect_nodes(&node_c, &root_node_e, true);
    assert_eq!(res_graph, graph);
}

#[test]
fn test_disconnect_nodes_method_fifth() {
    let graph = get_initial_graph();
    let mut res_graph = graph.clone();

    let node_k = LinkedGraphNode::from_name("k");
    let node_m = LinkedGraphNode::with_parents("m", vec![node_k.clone()]);

    res_graph.disconnect_nodes(&node_k, &node_m, true);
    assert_eq!(res_graph, graph);
}
