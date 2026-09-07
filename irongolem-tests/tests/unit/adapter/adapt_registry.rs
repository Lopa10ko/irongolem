use irongolem::golem::adapter::AdaptRegistry;
use irongolem::golem::dag::{Graph, GraphDelegate};
use test_support::fixtures::mock_adapter::MockAdapter;

fn domain_func_1arg(_g: test_support::fixtures::mock_adapter::MockDomainStructure) {}

fn domain_func_4args(
    _n: i32,
    _g: test_support::fixtures::mock_adapter::MockDomainStructure,
    _flag: bool,
    _struct2: test_support::fixtures::mock_adapter::MockDomainStructure,
) {
}

#[test]
fn test_adapt_1arg() {
    let adapter = MockAdapter;
    let opt_graph = adapter.adapt(test_support::fixtures::MockDomainStructure::new(vec![]));
    domain_func_1arg(adapter.restore(opt_graph.clone()));
    let _ = opt_graph;
}

#[test]
fn test_adapt_many_args() {
    let adapter = MockAdapter;
    let opt_graph = adapter.adapt(test_support::fixtures::MockDomainStructure::new(vec![]));
    domain_func_4args(
        4,
        adapter.restore(opt_graph.clone()),
        true,
        adapter.restore(opt_graph),
    );
}

#[test]
fn test_adapt_returned_same() {
    let adapter = MockAdapter;
    let opt_graph = adapter.adapt(test_support::fixtures::MockDomainStructure::new(vec![]));
    let restored = adapter.restore(opt_graph.clone());
    let returned = adapter.adapt(restored);
    assert_eq!(
        returned.as_ref().descriptive_id(),
        opt_graph.as_ref().descriptive_id()
    );
}

#[test]
fn test_adapt_returned_single() {
    fn domain_func_return1() -> test_support::fixtures::MockDomainStructure {
        test_support::fixtures::MockDomainStructure::new(vec![])
    }
    let _ = domain_func_return1();
}

#[test]
fn test_adapt_returned_many() {
    let adapter = MockAdapter;
    let opt_graph = adapter.adapt(test_support::fixtures::MockDomainStructure::new(vec![]));
    let _flag = true;
    let _graph = opt_graph;
    let _fitness = irongolem::golem::optimisers::fitness::Fitness::valid_fitness();
}

#[test]
fn test_adapt_registered_functions() {
    fn native_mutation(g: GraphDelegate) -> GraphDelegate {
        g
    }
    let f: fn(GraphDelegate) -> GraphDelegate = native_mutation;
    AdaptRegistry::register_native(f as usize);
    assert!(AdaptRegistry::is_native(f as usize));
    let adapter = MockAdapter;
    let opt_graph = adapter.adapt(test_support::fixtures::MockDomainStructure::new(vec![]));
    let mutated = native_mutation((*opt_graph).clone());
    let _ = mutated;
    assert!(AdaptRegistry::is_native(f as usize));
}

#[test]
fn test_adapt_unregistered_fail() {
    fn unregistered_mutation(g: GraphDelegate) -> GraphDelegate {
        // Distinct body so the compiler cannot merge this with other identity fns.
        let _ = g.length();
        g
    }
    let f: fn(GraphDelegate) -> GraphDelegate = unregistered_mutation;
    assert!(!AdaptRegistry::is_native(f as usize));
}
