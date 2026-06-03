//! custom

#[test]
fn test_custom_graph_opt() {
    // def test_custom_graph_opt():
    //     """Test checks for the use case of custom graph optimisation:
    //     that it can be initialised without problem and returns sane result."""
    //
    //     nodes_types = ['A', 'B', 'C', 'D']
    //     rules = [has_no_self_cycled_nodes]
    //
    //     requirements = GraphRequirements(
    //         num_of_generations=5,
    //         show_progress=False)
    //
    //     optimiser_parameters = GPAlgorithmParameters(
    //         pop_size=5,
    //         genetic_scheme_type=GeneticSchemeTypesEnum.steady_state,
    //         mutation_types=[
    //             MutationTypesEnum.simple,
    //             MutationTypesEnum.reduce,
    //             MutationTypesEnum.growth,
    //             MutationTypesEnum.local_growth],
    //         regularization_type=RegularizationTypesEnum.none)
    //
    //     graph_generation_params = GraphGenerationParams(
    //         adapter=DirectAdapter(CustomModel, CustomNode),
    //         rules_for_constraint=rules,
    //         node_factory=DefaultOptNodeFactory(available_node_types=nodes_types))
    //
    //     objective = Objective({'custom': custom_metric})
    //     initial_graphs = [graph_first(), graph_second(), graph_third(), graph_fourth(), graph_fifth()]
    //     init_population = InitialPopulationGenerator(optimiser_parameters.pop_size,
    //                                                  graph_generation_params, requirements)\
    //         .with_initial_graphs(initial_graphs)()
    //     optimiser = EvoGraphOptimizer(
    //         graph_generation_params=graph_generation_params,
    //         objective=objective,
    //         graph_optimizer_params=optimiser_parameters,
    //         requirements=requirements,
    //         initial_graphs=init_population)
    //
    //     objective_eval = ObjectiveEvaluate(objective)
    //     optimized_graphs = optimiser.optimise(objective_eval)
    //     optimized_network = optimiser.graph_generation_params.adapter.restore(optimized_graphs[0])
    //
    //     assert optimized_network is not None
    //     assert isinstance(optimized_network, CustomModel)
    //     assert isinstance(optimized_network.nodes[0], CustomNode)
    //     assert optimized_network.length > 1
    assert!(false);
}
