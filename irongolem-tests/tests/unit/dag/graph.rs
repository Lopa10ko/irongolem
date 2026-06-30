use irongolem::golem::dag::{
    Graph, GraphImpl, GraphNode, LinkedGraphNode, NodeContent, ReconnectType,
};

#[test]
fn test_graph_id() {
    let first = LinkedGraphNode::new(NodeContent::new("n1"));
    let second = LinkedGraphNode::with_parents("n2", vec![first.clone()]);
    let third = LinkedGraphNode::with_parents("n3", vec![first]);
    let final_node = LinkedGraphNode::with_parents("n4", vec![second, third]);
    let right_id = "((/n_n1;)/n_n2;;(/n_n1;)/n_n3;)/n_n4";
    assert_eq!(final_node.read().unwrap().descriptive_id(), right_id);
}

#[test]
fn test_delete_primary_node() {
    let first = LinkedGraphNode::new(NodeContent::new("n1"));
    let second = LinkedGraphNode::new(NodeContent::new("n2"));
    let third = LinkedGraphNode::with_parents("n3", vec![first.clone()]);
    let final_node = LinkedGraphNode::with_parents("n4", vec![second, third]);
    let mut graph = GraphImpl::new(final_node);
    graph.delete_node(&first, ReconnectType::Single);
    assert_eq!(graph.length(), 3);
}

#[test]
fn test_delete_intermediate_node() {
    let first = LinkedGraphNode::new(NodeContent::new("n1"));
    let second = LinkedGraphNode::new(NodeContent::new("n2"));
    let third = LinkedGraphNode::with_parents("n3", vec![first.clone()]);
    let final_node = LinkedGraphNode::with_parents("n4", vec![second, third.clone()]);
    let mut graph = GraphImpl::new(final_node);
    graph.delete_node(&third, ReconnectType::Single);
    assert_eq!(graph.depth(), 2);
}

#[test]
fn test_graph_copy_shallow() {
    let root = LinkedGraphNode::new(NodeContent::new("n1"));
    let graph = GraphImpl::new(root);
    let graph_copy = graph.clone();
    assert_ne!(graph.length(), 0);
    assert_eq!(graph.length(), graph_copy.length());
}
