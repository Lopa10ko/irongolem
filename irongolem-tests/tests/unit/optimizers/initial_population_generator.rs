//! initial_population_generator

#[test]
fn test_random_initial_population() {
    // def test_random_initial_population():
    //     requirements, graph_generation_params, initial_population_generator = setup_test(pop_size=3)
    //     generated_population = initial_population_generator()
    //     max_depth = requirements.max_depth
    //     verifier = graph_generation_params.verifier
    //     assert len(generated_population) == 3, \
    //         "If no initial graphs provided InitialPopulationGenerator returns randomly generated graphs."
    //     assert all(graph.depth <= max_depth for graph in generated_population)
    //     assert all(verifier(graph) for graph in generated_population)
    assert!(false);
}

#[test]
fn test_initial_graphs_as_initial_population() {
    // def test_initial_graphs_as_initial_population():
    //     adapter = DirectAdapter()
    //     initial_graphs = adapter.adapt([graph_first(), graph_second(), graph_third()])
    //
    //     requirements, graph_generation_params, initial_population_generator = setup_test(pop_size=3)
    //     initial_population_generator.with_initial_graphs(initial_graphs)
    //     generated_population = initial_population_generator()
    //     assert generated_population == initial_graphs
    //
    //     requirements, graph_generation_params, initial_population_generator = setup_test(pop_size=4)
    //     initial_population_generator.with_initial_graphs(initial_graphs)
    //     generated_population = initial_population_generator()
    //     assert generated_population == initial_graphs
    //
    //     requirements, graph_generation_params, initial_population_generator = setup_test(pop_size=2)
    //     initial_population_generator.with_initial_graphs(initial_graphs)
    //     generated_population = initial_population_generator()
    //     assert len(generated_population) == 2
    //     assert all(graph in initial_graphs for graph in generated_population)
    assert!(false);
}

#[test]
fn test_initial_population_generation_function() {
    // def test_initial_population_generation_function(pop_size):
    //     requirements, graph_generation_params, initial_population_generator = setup_test(pop_size=pop_size)
    //     initial_population_generator.with_custom_generation_function(
    //         lambda: choice([graph_first(), graph_second(), graph_third()]))
    //     verifier = graph_generation_params.verifier
    //
    //     generated_population = initial_population_generator()
    //     assert len(generated_population) <= 3
    //     assert all(verifier(graph) for graph in generated_population)
    //     unique = reduce(lambda l, x: l.append(x) or l if x not in l else l, generated_population, [])
    //     assert len(unique) == len(generated_population)
    assert!(false);
}
