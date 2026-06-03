//! crossover

#[test]
fn test_crossover_zero_probability() {
    // def test_crossover_zero_probability(crossover_type):
    //     graph_example_first = graph_first()
    //     graph_example_second = graph_second()
    //
    //     requirements = GraphRequirements()
    //     graph_generation_params = GraphGenerationParams(available_node_types=['a', 'b', 'c', 'd'])
    //     parameters = GPAlgorithmParameters(crossover_prob=0)
    //     crossover = Crossover(parameters, requirements, graph_generation_params)
    //
    //     crossover.parameters.crossover_types = [crossover_type]
    //     new_graphs = crossover([Individual(graph_example_first), Individual(graph_example_second)])
    //     assert new_graphs[0].graph == graph_example_first
    //     assert new_graphs[1].graph == graph_example_second
    assert!(false);
}

#[test]
fn test_crossover_none() {
    // def test_crossover_none():
    //     graph_example_first = graph_first()
    //     graph_example_second = graph_second()
    //
    //     requirements = GraphRequirements()
    //     graph_generation_params = GraphGenerationParams(available_node_types=['a', 'b', 'c', 'd'])
    //     opt_parameters = GPAlgorithmParameters(crossover_types=[CrossoverTypesEnum.none], crossover_prob=1)
    //     crossover = Crossover(opt_parameters, requirements, graph_generation_params)
    //     new_graphs = crossover([Individual(graph_example_first), Individual(graph_example_second)])
    //     assert new_graphs[0].graph == graph_example_first
    //     assert new_graphs[1].graph == graph_example_second
    assert!(false);
}

#[test]
fn test_crossover_exchange_edges() {
    // def test_crossover_exchange_edges():
    //     graph_example_first = graph_sixth()
    //     graph_example_second = graph_seventh()
    //     valid_graphs = [graph_example_first, graph_example_second, graph_eighth(), graph_ninth()]
    //
    //     new_graphs = exchange_edges_crossover(graph_example_first, graph_example_second, 2)
    //     assert any([new_graphs[0] == graph for graph in valid_graphs])
    //     assert any([new_graphs[1] == graph for graph in valid_graphs])
    assert!(false);
}

#[test]
fn test_crossover_exchange_parents_one() {
    // def test_crossover_exchange_parents_one():
    //     graph_example_first = graph_sixth()
    //     graph_example_second = graph_seventh()
    //     valid_graphs = [graph_example_first, graph_example_second]
    //
    //     new_graphs = exchange_parents_one_crossover(graph_example_first, graph_example_second, 2)
    //     assert any([new_graphs[0] == graph for graph in valid_graphs])
    //     assert any([new_graphs[1] == graph for graph in valid_graphs])
    assert!(false);
}

#[test]
fn test_crossover_exchange_parents_both() {
    // def test_crossover_exchange_parents_both():
    //     graph_example_first = graph_sixth()
    //     graph_example_second = graph_seventh()
    //     valid_graphs = [graph_example_first, graph_example_second]
    //
    //     new_graphs = exchange_parents_both_crossover(graph_example_first, graph_example_second, 2)
    //     assert any([new_graphs[0] == graph for graph in valid_graphs])
    //     assert any([new_graphs[1] == graph for graph in valid_graphs])
    assert!(false);
}

#[test]
fn test_crossover_with_single_node() {
    // def test_crossover_with_single_node(crossover_type):
    //     graph_example_first = graph_with_single_node()
    //     graph_example_second = graph_with_single_node()
    //
    //     requirements = GraphRequirements()
    //     graph_generation_params = GraphGenerationParams(available_node_types=['a', 'b', 'c', 'd'])
    //     parameters = GPAlgorithmParameters(crossover_prob=1)
    //     crossover = Crossover(parameters, requirements, graph_generation_params)
    //
    //     crossover.parameters.crossover_types = [crossover_type]
    //     new_graphs = crossover([Individual(graph_example_first), Individual(graph_example_second)])
    //     assert new_graphs[0].graph == graph_example_first
    //     assert new_graphs[1].graph == graph_example_second
    assert!(false);
}
