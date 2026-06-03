//! operators_reproducibility

#[test]
fn test_crossover_reproducibility() {
    // def test_crossover_reproducibility(crossover_type, seed):
    //     graph_example_first = graph_first()
    //     graph_example_second = graph_second()
    //
    //     parameters = GPAlgorithmParameters()
    //     requirements = GraphRequirements()
    //     graph_generation_params = GraphGenerationParams(available_node_types=['a', 'b', 'c', 'd'])
    //     crossover = Crossover(parameters, requirements, graph_generation_params)
    //
    //     crossover.parameters.crossover_types = [crossover_type]
    //
    //     def run_with_seed(seed):
    //         set_random_seed(seed)
    //         results = crossover([Individual(graph_example_first), Individual(graph_example_second)])
    //         results = [ind.graph.descriptive_id for ind in results]
    //         return results
    //
    //     results_first = run_with_seed(seed)
    //     results_second = run_with_seed(seed)
    //
    //     assert results_first == results_second
    assert!(false);
}

#[test]
fn test_mutation_reproducibility() {
    // def test_mutation_reproducibility(mutation_type, seed):
    //     params = get_mutation_params([mutation_type])
    //     mutation = Mutation(**params)
    //
    //     def run_with_seed(seed):
    //         set_random_seed(seed)
    //         ind = Individual(graph_first())
    //         new_ind = mutation(ind)
    //         if isinstance(new_ind, Individual):
    //             return new_ind.graph.descriptive_id
    //         else:
    //             return new_ind
    //
    //     results_first = run_with_seed(seed)
    //     results_second = run_with_seed(seed)
    //
    //     assert results_first == results_second
    assert!(false);
}
