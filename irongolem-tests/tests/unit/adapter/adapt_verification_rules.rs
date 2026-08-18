//! adapt_verification_rules

use irongolem::golem::dag::{default_dag_rules, GraphDelegate};
use test_support::fixtures::mock_adapter::{MockAdapter, MockDomainStructure, MockNode};

fn get_valid_graph() -> (
    std::sync::Arc<GraphDelegate>,
    MockDomainStructure,
    MockAdapter,
) {
    let first = MockNode::new("n1");
    let second = MockNode::with_parent("n2", first);
    let third = MockNode::with_parent("n3", second);
    let graph = MockDomainStructure::new(vec![third]);
    let adapter = MockAdapter;
    let opt_graph = adapter.adapt(graph.clone());
    (opt_graph, graph, adapter)
}

#[test]
fn test_adapt_verification_rules_dag() {
    // Python: DEFAULT_DAG_RULES are `@register_native`, so adapt_func must leave them
    // unchanged (`id(rule) == id(adapted_rule)`) and they must accept opt graphs.
    let (opt_graph, _graph, adapter) = get_valid_graph();
    for rule in default_dag_rules() {
        assert!(rule(opt_graph.as_ref()).is_ok());
        let adapted_rule = adapter.adapt_func(rule);
        assert!(adapted_rule(opt_graph.as_ref()).is_ok());
        assert_eq!(rule as usize, adapted_rule as usize);
    }
}
