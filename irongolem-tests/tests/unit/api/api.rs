//! api

#[test]
fn test_specifying_parameters_through_api() {
    // def test_specifying_parameters_through_api():
    //     """ Tests that parameters for optimizer are specified correctly. """
    //
    //     timeout = 1
    //     size = 16
    //     node_types = ('a', 'b')
    //     target_graph = generate_labeled_graph('tree', size, node_labels=node_types)
    //
    //     # Generate initial population with small tree graphs
    //     initial_graphs = [generate_labeled_graph('tree', 5, node_types) for _ in range(10)]
    //     # Setup objective: edit distance to target graph
    //     objective = Objective(partial(tree_edit_dist, target_graph))
    //
    //     golem = GOLEM(timeout=timeout,
    //                   logging_level=logging.INFO,
    //                   early_stopping_iterations=100,
    //                   initial_graphs=initial_graphs,
    //                   objective=objective,
    //                   genetic_scheme_type=GeneticSchemeTypesEnum.parameter_free,
    //                   max_pop_size=50,
    //                   mutation_types=[MutationTypesEnum.single_add,
    //                                   MutationTypesEnum.single_drop,
    //                                   MutationTypesEnum.single_change],
    //                   crossover_types=[CrossoverTypesEnum.subtree],
    //                   available_node_types=node_types  # Node types that can appear in graphs
    //                   )
    //
    //     # setup with externally specifying params
    //     requirements = GraphRequirements(
    //         early_stopping_iterations=100,
    //         timeout=datetime.timedelta(minutes=timeout),
    //         n_jobs=1,
    //     )
    //     gp_params = GPAlgorithmParameters(
    //         genetic_scheme_type=GeneticSchemeTypesEnum.parameter_free,
    //         max_pop_size=50,
    //         mutation_types=[MutationTypesEnum.single_add,
    //                         MutationTypesEnum.single_drop,
    //                         MutationTypesEnum.single_change],
    //         crossover_types=[CrossoverTypesEnum.subtree]
    //     )
    //     graph_gen_params = GraphGenerationParams(
    //         adapter=BaseNetworkxAdapter(),  # Example works with NetworkX graphs
    //         rules_for_constraint=DEFAULT_DAG_RULES,  # We don't want cycles in the graph
    //         available_node_types=node_types  # Node types that can appear in graphs
    //     )
    //
    //     assert golem.gp_algorithm_parameters == gp_params
    //     # compared by pickle dump since there are lots of inner classes with not implemented __eq__ magic methods
    //     # probably needs to be fixed
    //     assert pickle.dumps(golem.graph_generation_parameters) == pickle.dumps(graph_gen_params)
    //     # need to be compared by dicts since the classes itself are different
    //     assert golem.graph_requirements.__dict__ == requirements.__dict__
    assert!(false);
}
