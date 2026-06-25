use irongolem::dag::{LinkedGraph, ReconnectType};

#[test]
fn test_graph_id() {
    let mut g = LinkedGraph::new();
    let first = g.add_detached("n1", &[]);
    let second = g.add_detached("n2", &[first]);
    let third = g.add_detached("n3", &[first]);
    let final_node = g.add_detached("n4", &[second, third]);
    let right_id = "((/n_n1;)/n_n2;;(/n_n1;)/n_n3;)/n_n4";
    assert_eq!(g.node_descriptive_id(final_node), right_id);
}

#[test]
fn test_delete_primary_node() {
    let mut g = LinkedGraph::new();
    let first = g.add_detached("n1", &[]);
    let second = g.add_detached("n2", &[]);
    let third = g.add_detached("n3", &[first]);
    let final_node = g.add_detached("n4", &[second, third]);
    g.add_node(final_node);

    g.delete_node(first, ReconnectType::Single);
    assert_eq!(g.length(), 3);
}

#[test]
fn test_delete_intermediate_node() {
    let mut g = LinkedGraph::new();
    let first = g.add_detached("n1", &[]);
    let second = g.add_detached("n2", &[]);
    let third = g.add_detached("n3", &[first]);
    let final_node = g.add_detached("n4", &[second, third]);
    g.add_node(final_node);

    g.delete_node(third, ReconnectType::Single);
    assert_eq!(g.depth(), 2);
}

#[test]
fn test_graph_copy_shallow() {
    let mut g = LinkedGraph::new();
    let root = g.add_detached("n1", &[]);
    g.add_node(root);

    let graph_copy = g.clone();
    assert_ne!(g.length(), 0);
    assert_eq!(g.length(), graph_copy.length());
}
