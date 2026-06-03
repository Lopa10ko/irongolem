//! adapt_registry

#[test]
fn test_adapt_1arg() {
    // def test_adapt_1arg():
    //     opt_graph, dom_struct = get_graphs()
    //
    //     func = domain_func_1arg
    //     restored_func = adapt(func)
    //
    //     # test that opt graph is accepted by restored domain function
    //     restored_func(opt_graph)
    assert!(false);
}

#[test]
fn test_adapt_many_args() {
    // def test_adapt_many_args():
    //     opt_graph, dom_struct = get_graphs()
    //
    //     func = domain_func_4args
    //     restored_func = adapt(func)
    //
    //     func(4, dom_struct, flag=True, struct2=dom_struct)
    //     # test that opt graph is accepted by restored domain function
    //     restored_func(4, opt_graph, flag=True, struct2=opt_graph)
    assert!(false);
}

#[test]
fn test_adapt_returned_same() {
    // def test_adapt_returned_same():
    //     opt_graph, dom_struct = get_graphs()
    //
    //     func = domain_func_return_same
    //     restored_func = adapt(func)
    //
    //     # sanity check
    //     returned_graph = func(dom_struct)
    //     assert graphs_same(returned_graph, dom_struct)
    //
    //     returned_graph = restored_func(opt_graph)
    //     assert graphs_same(returned_graph, opt_graph)
    //     # NB: identity of the graphs is not preserved
    //     assert id(returned_graph) != id(opt_graph)
    assert!(false);
}

#[test]
fn test_adapt_returned_single() {
    // def test_adapt_returned_single():
    //     func = domain_func_return1
    //     restored_func = adapt(func)
    //
    //     return_value = restored_func()
    //     assert isinstance(return_value, OptGraph)
    assert!(false);
}

#[test]
fn test_adapt_returned_many() {
    // def test_adapt_returned_many():
    //     opt_graph, dom_struct = get_graphs()
    //
    //     func = domain_func_return3
    //     restored_func = adapt(func)
    //
    //     flag, graph, fitness = restored_func(opt_graph)
    //     # test that return value is adapted back to opt graph (if return is present)
    //     assert isinstance(graph, OptGraph)
    //     # and that other values are left unchanged
    //     assert isinstance(flag, bool)
    //     assert isinstance(fitness, Fitness)
    assert!(false);
}

#[test]
fn test_adapt_registered_functions() {
    // def test_adapt_registered_functions(mutation):
    //     """Demonstrates how both native & domain mutations are handled
    //     uniformly by adapter registry thanks to @register_native decorator."""
    //
    //     opt_graph, dom_struct = get_graphs()
    //
    //     restored_mutation = adapt(mutation)
    //     mutated_graph = restored_mutation(opt_graph)
    //
    //     assert isinstance(mutated_graph, OptGraph)
    //     assert not graphs_same(mutated_graph, opt_graph)
    assert!(false);
}

#[test]
fn test_adapt_unregistered_fail() {
    // def test_adapt_unregistered_fail(mutation):
    //     opt_graph, dom_struct = get_graphs()
    //
    //     was_registered = AdaptRegistry().is_native(mutation)
    //     if was_registered:
    //         AdaptRegistry().unregister_native(mutation)  # clear registration by previous tests
    //
    //     restored_mutation = adapt(mutation)
    //
    //     with pytest.raises(TypeError):
    //         restored_mutation(dom_struct)
    //     with pytest.raises(TypeError):
    //         restored_mutation(opt_graph)
    //
    //     if was_registered:
    //         AdaptRegistry().register_native(mutation)
    assert!(false);
}
