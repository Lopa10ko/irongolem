use std::collections::BTreeMap;

use irongolem::dag::{LinkedGraph, NodeContent};
use serde_json::json;

#[test]
fn test_node_description() {
    let operation_type = "logit";
    let mut g = LinkedGraph::new();
    let node = g.add_detached(NodeContent::new(operation_type), &[]);
    let expected = format!("n_{operation_type}");
    assert_eq!(g.description(node), expected);
}

#[test]
fn test_node_description_with_params() {
    let operation_type = "logit";
    let mut params = BTreeMap::new();
    params.insert("some_param".to_string(), json!(10));
    let mut g = LinkedGraph::new();
    let node = g.add_detached(NodeContent::with_params(operation_type, params), &[]);
    // Params are rendered with `{:?}` over the (sorted) BTreeMap of serde_json values.
    assert_eq!(g.description(node), "n_logit_{\"some_param\": Number(10)}");
}
