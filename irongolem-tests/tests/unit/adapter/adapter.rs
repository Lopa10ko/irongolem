//! adapter

#[test]
fn test_adapters_params_correct() {
    // def test_adapters_params_correct(adapter, graph_with_params):
    //     """ Checking the correct conversion of hyperparameters in nodes when nodes
    //     are passing through adapter
    //     """
    //     init_alpha = 12.1
    //     graph = graph_with_params(init_alpha)
    //
    //     # Convert into OptGraph object
    //     opt_graph = adapter.adapt(graph)
    //     assert np.isclose(init_alpha, opt_graph.root_node.parameters['alpha'])
    //     # Get graph object back
    //     restored_graph = adapter.restore(opt_graph)
    //     # Get hyperparameter value after graph restoration
    //     if isinstance(graph, Graph):
    //         restored_alpha = restored_graph.root_node.content['params']['alpha']
    //     else:
    //         root_node = [node for node in restored_graph.nodes() if restored_graph.out_degree(node) == 0][0]
    //         restored_alpha = restored_graph.nodes[root_node]['alpha']
    //     assert np.isclose(init_alpha, restored_alpha)
    assert!(false);
}

#[test]
fn test_restored_and_adapted_are_equal() {
    // def test_restored_and_adapted_are_equal(adapter, optgraph):
    //     graph = adapter.restore(optgraph)
    //     retransformed_optgraph = adapter.adapt(graph)
    //
    //     # assert 2-way mapping doesn't change the structure
    //     assert retransformed_optgraph.descriptive_id == optgraph.descriptive_id
    //     # assert that new graph is a different object
    //     assert id(optgraph) != id(retransformed_optgraph)
    assert!(false);
}

#[test]
fn test_graph_adapt_properly() {
    // def test_graph_adapt_properly(adapter, graph):
    //     verifier = GraphVerifier(DEFAULT_DAG_RULES)
    //
    //     assert all(isinstance(node, MockNode) for node in graph.nodes)
    //     assert _check_nodes_references_correct(graph)
    //     assert verifier(graph)
    //
    //     opt_graph = adapter.adapt(graph)
    //
    //     assert all(type(node) is OptNode for node in opt_graph.nodes)  # checking strict type equality!
    //     assert _check_nodes_references_correct(opt_graph)
    //     assert verifier(opt_graph)
    assert!(false);
}

#[test]
fn test_adapted_has_same_structure() {
    // def test_adapted_has_same_structure(adapter, graph):
    //     opt_graph = adapter.adapt(graph)
    //
    //     # assert graph structures are same
    //     assert graph.descriptive_id == opt_graph.descriptive_id
    assert!(false);
}

#[test]
fn test_adapted_and_restored_are_equal() {
    // def test_adapted_and_restored_are_equal(adapter, graph):
    //     opt_graph = adapter.adapt(graph)
    //     restored_graph = adapter.restore(opt_graph)
    //
    //     # assert 2-way mapping doesn't change the structure
    //     assert graph.descriptive_id == restored_graph.descriptive_id
    //     # assert that new graph is a different object
    //     assert id(graph) != id(restored_graph)
    assert!(false);
}

#[test]
fn test_changes_to_transformed_dont_affect_origin() {
    // def test_changes_to_transformed_dont_affect_origin(adapter, graph):
    //     original_graph = deepcopy(graph)
    //     opt_graph = adapter.adapt(graph)
    //
    //     # before change they're equal
    //     assert graph.descriptive_id == opt_graph.descriptive_id
    //
    //     changed_node = choice(opt_graph.nodes)
    //     changed_node.content['name'] = 'another_operation'
    //
    //     # assert that changes to the adapted graph don't affect original graph
    //     assert graph.descriptive_id != opt_graph.descriptive_id
    //     assert graph.descriptive_id == original_graph.descriptive_id
    //
    //     original_opt_graph = deepcopy(opt_graph)
    //     restored_graph = adapter.restore(opt_graph)
    //
    //     # before change they're equal
    //     assert opt_graph.descriptive_id == restored_graph.descriptive_id
    //
    //     changed_node = choice(restored_graph.nodes)
    //     changed_node.content['name'] = 'yet_another_operation'
    //
    //     # assert that changes to the restored graph don't affect original graph
    //     assert opt_graph.descriptive_id != restored_graph.descriptive_id
    //     assert opt_graph.descriptive_id == original_opt_graph.descriptive_id
    assert!(false);
}

#[test]
fn test_no_opt_or_graph_nodes_after_adapt_so_complex_graph() {
    // def test_no_opt_or_graph_nodes_after_adapt_so_complex_graph():
    //     adapter = MockAdapter()
    //     graph = get_complex_graph()
    //     adapter.adapt(graph)
    //
    //     assert not find_first(graph, lambda n: type(n) in (GraphNode, OptNode))
    assert!(false);
}
