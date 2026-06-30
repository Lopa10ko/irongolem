//! gp_operators

use irongolem::golem::adapter::DirectAdapter;
use irongolem::golem::dag::{nodes_from_layer, Graph};
use irongolem::golem::optimisers::fitness::{Fitness, MultiObjFitness};
use irongolem::golem::optimisers::genetic::constants::MIN_POP_SIZE;
use irongolem::golem::optimisers::genetic::rng::GeneticRng;
use irongolem::golem::optimisers::genetic::{
    equivalent_subtree, filter_duplicates, get_structure_unique_population, replace_subtrees,
};
use irongolem::golem::optimisers::history::{Individual, ParetoFront};
use test_support::fixtures::{
    get_graph_with_operation, graph_first, graph_fourth, graph_second, graph_third,
    graph_with_multi_roots_first, graph_with_multi_roots_second, graphs_same, identity_evaluator,
    population_with_structural_duplicates,
};

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
    let mut archive = ParetoFront::new(3);
    let adapter = DirectAdapter;
    let archive_graphs = [graph_first(), graph_second(), graph_third()];
    let mut archive_items: Vec<Individual> = archive_graphs
        .into_iter()
        .map(|g| Individual::new(adapter.adapt(g)))
        .collect();
    let pop_graphs = [graph_first(), graph_second(), graph_third(), graph_fourth()];
    let mut population: Vec<Individual> = pop_graphs
        .into_iter()
        .map(|g| Individual::new(adapter.adapt(g)))
        .collect();
    let archive_items_fitness = [(0.80001, 0.25), (0.7, 0.1), (0.9, 0.7)];
    let population_fitness = [(0.8, 0.25), (0.59, 0.25), (0.9, 0.7), (0.7, 0.1)];
    let weights = [-1.0, 1.0];
    for (ind, fitness_vals) in archive_items.iter_mut().zip(archive_items_fitness) {
        ind.set_fitness(Fitness::Multi(MultiObjFitness::new(
            &[fitness_vals.0, fitness_vals.1],
            Some(&weights),
        )));
    }
    for (ind, fitness_vals) in population.iter_mut().zip(population_fitness) {
        ind.set_fitness(Fitness::Multi(MultiObjFitness::new(
            &[fitness_vals.0, fitness_vals.1],
            Some(&weights),
        )));
    }
    archive.update(archive_items);
    let filtered_archive = filter_duplicates(&archive, &population);
    assert_eq!(filtered_archive.len(), 1);
    assert_eq!(filtered_archive[0].fitness.values()[0], -0.80001);
    assert_eq!(filtered_archive[0].fitness.values()[1], 0.25);
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
    let mut graph_1 = graph_first();
    let passed_graph_1 = graph_1.deep_clone();
    let mut graph_2 = graph_third();
    let layer_in_first = graph_1.depth() - 1;
    let layer_in_second = 0;
    let max_depth = 3;
    let node_from_graph_first = nodes_from_layer(&graph_1, layer_in_first)[0].clone();
    let node_from_graph_second = nodes_from_layer(&graph_2, layer_in_second)[0].clone();
    replace_subtrees(
        &mut graph_1,
        &mut graph_2,
        &node_from_graph_first,
        &node_from_graph_second,
        layer_in_first,
        layer_in_second,
        max_depth,
    );
    assert!(graph_1.depth() <= max_depth);
    assert_eq!(graph_1, passed_graph_1);
    assert!(graph_2.depth() <= max_depth);
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
    let cases: [(
        fn() -> irongolem::golem::dag::GraphDelegate,
        fn() -> irongolem::golem::dag::GraphDelegate,
        usize,
        usize,
    ); 4] = [
        (graph_first, graph_second, 4, 24),
        (graph_first, graph_third, 0, 12),
        (graph_second, graph_third, 0, 15),
        (graph_third, graph_third, 1, 10),
    ];
    for (g1, g2, answer_primary, answer_non_primary) in cases {
        let graph_1 = g1();
        let graph_2 = g2();
        let without_primary = equivalent_subtree(&graph_1, &graph_2, false);
        assert_eq!(without_primary.len(), answer_primary);
        let with_primary = equivalent_subtree(&graph_1, &graph_2, true);
        assert_eq!(with_primary.len(), answer_non_primary);
    }
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
    let graph_first = graph_with_multi_roots_first();
    let graph_second = graph_with_multi_roots_second();
    let without_primary = equivalent_subtree(&graph_first, &graph_second, false);
    assert_eq!(without_primary.len(), 2);
    let with_primary = equivalent_subtree(&graph_first, &graph_second, true);
    assert_eq!(with_primary.len(), 8);
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
    let operations = ["a", "b", "c", "d", "e"];
    let population_with_reps = population_with_structural_duplicates(&operations);
    let adapter = DirectAdapter;
    let new_population = get_structure_unique_population(
        population_with_reps,
        identity_evaluator(),
        &GeneticRng::seeded(42),
    );
    assert_eq!(new_population.len(), operations.len());
    for op in operations {
        let target = adapter.adapt(get_graph_with_operation(op));
        assert!(new_population
            .iter()
            .any(|ind| graphs_same(&ind.graph, &target)));
    }
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
    let operations = ["a", "b", "c"];
    let population_with_reps = population_with_structural_duplicates(&operations);
    let new_population = get_structure_unique_population(
        population_with_reps,
        identity_evaluator(),
        &GeneticRng::seeded(42),
    );
    assert_eq!(new_population.len(), MIN_POP_SIZE);
}
