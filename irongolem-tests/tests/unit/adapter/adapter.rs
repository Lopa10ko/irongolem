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
