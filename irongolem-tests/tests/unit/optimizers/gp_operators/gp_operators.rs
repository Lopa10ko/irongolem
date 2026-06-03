//! gp_operators

#[test]
fn test_filter_duplicates() {
    // def test_filter_duplicates():
    //     archive = ParetoFront(maxsize=3)
    //     adapter = DirectAdapter()
    //
    //     archive_items = [Individual(adapter.adapt(g)) for g in [graph_first(), graph_second(), graph_third()]]
    //     population = [Individual(adapter.adapt(g)) for g in [graph_first(), graph_second(),
    //                                                          graph_third(), graph_fourth()]]
    //     archive_items_fitness = ((0.80001, 0.25), (0.7, 0.1), (0.9, 0.7))
    //     population_fitness = ((0.8, 0.25), (0.59, 0.25), (0.9, 0.7), (0.7, 0.1))
    //     weights = (-1, 1)
    //     for ind_num in range(len(archive_items)):
    //         archive_items[ind_num].set_evaluation_result(
    //             MultiObjFitness(values=archive_items_fitness[ind_num], weights=weights))
    //     for ind_num in range(len(population)):
    //         population[ind_num].set_evaluation_result(MultiObjFitness(values=population_fitness[ind_num], weights=weights))
    //     archive.update(archive_items)
    //     filtered_archive = filter_duplicates(archive, population)
    //     assert len(filtered_archive) == 1
    //     assert filtered_archive[0].fitness.values[0] == -0.80001
    //     assert filtered_archive[0].fitness.values[1] == 0.25
    assert!(false);
}

#[test]
fn test_replace_subtree() {
    // def test_replace_subtree():
    //     # graph with depth = 3
    //     graph_1 = graph_first()
    //     passed_graph_1 = deepcopy(graph_1)
    //     # graph with depth = 2
    //     graph_2 = graph_third()
    //
    //     # choose the first layer of the first graph
    //     layer_in_first = graph_1.depth - 1
    //     # choose the last layer of the second graph
    //     layer_in_second = 0
    //     max_depth = 3
    //
    //     node_from_graph_first = nodes_from_layer(graph_1, layer_in_first)[0]
    //     node_from_graph_second = nodes_from_layer(graph_2, layer_in_second)[0]
    //
    //     # replace_subtrees must not replace subgraph in the first graph and its depth must be <= max_depth
    //     replace_subtrees(graph_1, graph_2, node_from_graph_first, node_from_graph_second,
    //                      layer_in_first, layer_in_second, max_depth)
    //     assert graph_1.depth <= max_depth
    //     assert graph_1 == passed_graph_1
    //     assert graph_2.depth <= max_depth
    assert!(false);
}

#[test]
fn test_graphs_equivalent_subtree() {
    // def test_graphs_equivalent_subtree(graphs_to_search_in, subgraphs_counts):
    //     graph_1, graph_2 = graphs_to_search_in
    //     answer_primary, answer_non_primary = subgraphs_counts
    //
    //     # get all common subgraphs. primary nodes are not considered.
    //     similar_nodes_first_and_second = equivalent_subtree(graph_first=graph_1, graph_second=graph_2,
    //                                                         with_primary_nodes=False)
    //     assert len(similar_nodes_first_and_second) == answer_primary
    //
    //     # get all common subgraphs. primary nodes are considered too.
    //     similar_nodes_first_and_second = equivalent_subtree(graph_first=graph_1, graph_second=graph_2,
    //                                                         with_primary_nodes=True)
    //     assert len(similar_nodes_first_and_second) == answer_non_primary
    assert!(false);
}

#[test]
fn test_graphs_with_multi_root_equivalent_subtree() {
    // def test_graphs_with_multi_root_equivalent_subtree():
    //     graph_first = graph_with_multi_roots_first()
    //     graph_second = graph_with_multi_roots_second()
    //
    //     # get all common subgraphs. primary nodes are not considered.
    //     similar_nodes_first_and_second = equivalent_subtree(graph_first=graph_first, graph_second=graph_second,
    //                                                         with_primary_nodes=False)
    //     assert len(similar_nodes_first_and_second) == 2
    //
    //     # get all common subgraphs. primary nodes are considered too.
    //     similar_nodes_first_and_second = equivalent_subtree(graph_first=graph_first, graph_second=graph_second,
    //                                                         with_primary_nodes=True)
    //     assert len(similar_nodes_first_and_second) == 8
    assert!(false);
}

#[test]
fn test_structural_diversity() {
    // def test_structural_diversity():
    //     """ Checks if `get_structure_unique_population` method returns population without structural duplicates. """
    //     operations = ['a', 'b', 'c', 'd', 'e']
    //     population_with_reps = population_with_structural_duplicates(operations=operations)
    //     optimizer, objective = set_up_optimizer(operations=operations)
    //
    //     adapter = DirectAdapter()
    //     evaluator = SequentialDispatcher(adapter).dispatch(objective)
    //     new_population = optimizer.get_structure_unique_population(population_with_reps, evaluator)
    //
    //     target_new_population = []
    //     for op in operations:
    //         target_new_population += [Individual(adapter.adapt(get_graph_with_operation(operation=op)))]
    //
    //     for i in range(len(target_new_population)):
    //         assert graphs_same(new_population[i].graph, target_new_population[i].graph)
    assert!(false);
}

#[test]
fn test_recover_pop_size_after_structure_check() {
    // def test_recover_pop_size_after_structure_check():
    //     """ Checks that `get_structure_unique_population` extends population
    //     if after structural check there sre less than MIN_POP_SIZE individuals in population. """
    //     operations = ['a', 'b', 'c']
    //     population_with_reps = population_with_structural_duplicates(operations=operations)
    //     optimizer, objective = set_up_optimizer(operations=operations)
    //     adapter = DirectAdapter()
    //     evaluator = SequentialDispatcher(adapter).dispatch(objective)
    //     new_population = optimizer.get_structure_unique_population(population_with_reps, evaluator)
    //
    //     assert len(new_population) == MIN_POP_SIZE
    assert!(false);
}
