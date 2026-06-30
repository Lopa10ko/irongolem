use std::collections::HashMap;

use irongolem::golem::dag::{GraphNode, LinkedGraphNode, NodeContent};
use serde_json::json;

#[test]
fn test_node_description() {
    let operation_type = "logit";
    let node = LinkedGraphNode::new(NodeContent::new(operation_type));
    let expected = format!("n_{operation_type}");
    assert_eq!(node.read().unwrap().description(), expected);
}

#[test]
fn test_node_description_with_params() {
    let operation_type = "logit";
    let mut params = HashMap::new();
    params.insert("some_param".to_string(), json!(10));
    let node = LinkedGraphNode::new(NodeContent::with_params(operation_type, params.clone()));
    let expected = format!("n_{operation_type}_{params:?}");
    assert_eq!(node.read().unwrap().description(), expected);
}
