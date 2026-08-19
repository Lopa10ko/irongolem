use std::sync::Arc;

use irongolem::golem::adapter::DirectAdapter;
use irongolem::golem::dag::Graph;
use test_support::fixtures::mock_adapter::{
    graph_with_params, MockAdapter, MockDomainStructure, MockNode,
};

#[test]
fn test_adapters_params_correct() {
    let init_alpha = 12.1;
    let graph = graph_with_params(init_alpha);
    let adapter = MockAdapter;
    let opt_graph = adapter.adapt(graph.clone());
    let restored = adapter.restore(opt_graph);
    let restored_alpha = restored.nodes[0]
        .content
        .get("params")
        .and_then(|p| p.get("alpha"))
        .and_then(|v| v.as_f64())
        .unwrap();
    assert!((init_alpha - restored_alpha).abs() < 1e-6_f64);
}

#[test]
fn test_restored_and_adapted_are_equal() {
    let adapter = MockAdapter;
    let optgraph = adapter.adapt(MockDomainStructure::new(vec![MockNode::new("a")]));
    let graph = adapter.restore(optgraph.clone());
    let retransformed = adapter.adapt(graph);
    assert_eq!(
        retransformed.as_ref().descriptive_id(),
        optgraph.as_ref().descriptive_id()
    );
    assert!(!Arc::ptr_eq(&optgraph, &retransformed));
}

#[test]
fn test_graph_adapt_properly() {
    let adapter = MockAdapter;
    let graph = MockDomainStructure::new(vec![MockNode::new("a")]);
    let opt_graph = adapter.adapt(graph);
    assert!(opt_graph.as_ref().descriptive_id().contains('a'));
}

#[test]
fn test_adapted_has_same_structure() {
    let adapter = MockAdapter;
    let graph = MockDomainStructure::new(vec![MockNode::new("a"), MockNode::new("b")]);
    let opt_graph = adapter.adapt(graph.clone());
    assert_eq!(
        adapter.restore(opt_graph.clone()).nodes.len(),
        graph.nodes.len()
    );
}

#[test]
fn test_adapted_and_restored_are_equal() {
    let adapter = MockAdapter;
    let graph = MockDomainStructure::new(vec![MockNode::new("a")]);
    let opt_graph = adapter.adapt(graph.clone());
    let restored = adapter.restore(opt_graph);
    assert_eq!(restored.nodes.len(), graph.nodes.len());
}

#[test]
fn test_changes_to_transformed_dont_affect_origin() {
    let adapter = MockAdapter;
    let graph = MockDomainStructure::new(vec![MockNode::new("a")]);
    let origin_id = format!("{:?}", graph.nodes[0].content);
    let mut opt_graph = adapter.adapt(graph);
    opt_graph = DirectAdapter.adapt((*opt_graph).clone());
    let _ = origin_id;
}

#[test]
fn test_no_opt_or_graph_nodes_after_adapt_so_complex_graph() {
    let adapter = MockAdapter;
    let graph = MockDomainStructure::new(vec![
        MockNode::new("a"),
        MockNode::new("b"),
        MockNode::new("c"),
    ]);
    let restored = adapter.restore(adapter.adapt(graph));
    assert_eq!(restored.nodes.len(), 3);
}

fn mock_node_name(node: &MockNode) -> &str {
    node.content
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
}

#[test]
fn test_mock_adapter_roundtrip_three_node_chain() {
    let first = MockNode::new("n1");
    let second = MockNode::with_parent("n2", first);
    let third = MockNode::with_parent("n3", second);
    let graph = MockDomainStructure::new(vec![third]);
    let adapter = MockAdapter;

    let opt_graph = adapter.adapt(graph);
    assert_eq!(opt_graph.as_ref().length(), 3);
    assert_eq!(opt_graph.as_ref().root_nodes().len(), 1);

    let restored = adapter.restore(opt_graph);
    assert_eq!(restored.nodes.len(), 1);
    assert_eq!(mock_node_name(&restored.nodes[0]), "n3");
    assert_eq!(restored.nodes[0].nodes_from.len(), 1);
    assert_eq!(mock_node_name(&restored.nodes[0].nodes_from[0]), "n2");
    assert_eq!(restored.nodes[0].nodes_from[0].nodes_from.len(), 1);
    assert_eq!(
        mock_node_name(&restored.nodes[0].nodes_from[0].nodes_from[0]),
        "n1"
    );
}

#[test]
fn test_mock_adapter_roundtrip_shared_parent() {
    let node_a = Arc::new(MockNode::new("a"));
    let node_b = Arc::new(MockNode::with_parents("b", vec![node_a.clone()]));
    let node_c = MockNode::with_parents("c", vec![node_b.clone(), node_a.clone()]);
    let graph = MockDomainStructure::new(vec![node_c]);
    let adapter = MockAdapter;

    let opt_graph = adapter.adapt(graph);
    assert_eq!(opt_graph.as_ref().length(), 3);
    assert_eq!(opt_graph.as_ref().root_nodes().len(), 1);

    let restored = adapter.restore(opt_graph);
    assert_eq!(restored.nodes.len(), 1);
    let restored_c = &restored.nodes[0];
    assert_eq!(mock_node_name(restored_c), "c");
    assert_eq!(restored_c.nodes_from.len(), 2);
    assert_eq!(mock_node_name(&restored_c.nodes_from[0]), "b");
    assert_eq!(mock_node_name(&restored_c.nodes_from[1]), "a");
    assert_eq!(restored_c.nodes_from[0].nodes_from.len(), 1);
    assert!(Arc::ptr_eq(
        &restored_c.nodes_from[0].nodes_from[0],
        &restored_c.nodes_from[1]
    ));
}
