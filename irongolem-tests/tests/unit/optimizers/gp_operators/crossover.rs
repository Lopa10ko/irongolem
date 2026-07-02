//! crossover

use std::sync::Arc;

use irongolem::golem::optimisers::genetic::operators::crossover::{
    exchange_edges_crossover, exchange_parents_both_crossover, exchange_parents_one_crossover,
    Crossover, CrossoverTypesEnum,
};
use irongolem::golem::optimisers::genetic::params::{
    GPAlgorithmParameters, GraphGenerationParams, GraphRequirements,
};
use irongolem::golem::optimisers::genetic::rng::GeneticRng;
use irongolem::golem::optimisers::history::Individual;
use test_support::fixtures::{
    graph_eighth, graph_first, graph_ninth, graph_second, graph_seventh, graph_sixth,
    graph_with_single_node, graphs_same,
};

const ALL_CROSSOVER_TYPES: [CrossoverTypesEnum; 7] = [
    CrossoverTypesEnum::Subtree,
    CrossoverTypesEnum::OnePoint,
    CrossoverTypesEnum::None,
    CrossoverTypesEnum::Subgraph,
    CrossoverTypesEnum::ExchangeEdges,
    CrossoverTypesEnum::ExchangeParentsOne,
    CrossoverTypesEnum::ExchangeParentsBoth,
];

fn graph_generation_params() -> GraphGenerationParams {
    GraphGenerationParams::new(
        vec!["a", "b", "c", "d"]
            .into_iter()
            .map(String::from)
            .collect(),
    )
}

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
    for crossover_type in ALL_CROSSOVER_TYPES {
        let graph_example_first = graph_first();
        let graph_example_second = graph_second();
        let requirements = GraphRequirements::default();
        let graph_gen_params = graph_generation_params();
        let mut parameters = GPAlgorithmParameters::default().with_crossover_prob(0.0);
        parameters.crossover_types = vec![crossover_type];
        let crossover = Crossover::new(parameters, requirements, graph_gen_params);
        let new_graphs = crossover.call(vec![
            Individual::new(Arc::new(graph_example_first.clone())),
            Individual::new(Arc::new(graph_example_second.clone())),
        ]);
        assert!(graphs_same(&new_graphs[0].graph, &graph_example_first));
        assert!(graphs_same(&new_graphs[1].graph, &graph_example_second));
    }
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
    let graph_example_first = graph_first();
    let graph_example_second = graph_second();
    let requirements = GraphRequirements::default();
    let graph_gen_params = graph_generation_params();
    let parameters = GPAlgorithmParameters::default()
        .with_crossover_types(vec![CrossoverTypesEnum::None])
        .with_crossover_prob(1.0);
    let crossover = Crossover::new(parameters, requirements, graph_gen_params);
    let new_graphs = crossover.call(vec![
        Individual::new(Arc::new(graph_example_first.clone())),
        Individual::new(Arc::new(graph_example_second.clone())),
    ]);
    assert!(graphs_same(&new_graphs[0].graph, &graph_example_first));
    assert!(graphs_same(&new_graphs[1].graph, &graph_example_second));
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
    let rng = GeneticRng::seeded(42);
    let graph_example_first = graph_sixth();
    let graph_example_second = graph_seventh();
    let valid_graphs = [
        graph_example_first.clone(),
        graph_example_second.clone(),
        graph_eighth(),
        graph_ninth(),
    ];
    let mut g1 = graph_example_first;
    let mut g2 = graph_example_second;
    exchange_edges_crossover(&mut g1, &mut g2, 2, &rng);
    assert!(valid_graphs.iter().any(|g| graphs_same(&g1, g)));
    assert!(valid_graphs.iter().any(|g| graphs_same(&g2, g)));
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
    let rng = GeneticRng::seeded(42);
    let graph_example_first = graph_sixth();
    let graph_example_second = graph_seventh();
    let valid_graphs = [graph_example_first.clone(), graph_example_second.clone()];
    let mut g1 = graph_example_first;
    let mut g2 = graph_example_second;
    exchange_parents_one_crossover(&mut g1, &mut g2, 2, &rng);
    assert!(valid_graphs.iter().any(|g| graphs_same(&g1, g)));
    assert!(valid_graphs.iter().any(|g| graphs_same(&g2, g)));
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
    let rng = GeneticRng::seeded(42);
    let graph_example_first = graph_sixth();
    let graph_example_second = graph_seventh();
    let valid_graphs = [graph_example_first.clone(), graph_example_second.clone()];
    let mut g1 = graph_example_first;
    let mut g2 = graph_example_second;
    exchange_parents_both_crossover(&mut g1, &mut g2, 2, &rng);
    assert!(valid_graphs.iter().any(|g| graphs_same(&g1, g)));
    assert!(valid_graphs.iter().any(|g| graphs_same(&g2, g)));
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
    for crossover_type in ALL_CROSSOVER_TYPES {
        let mut parameters = GPAlgorithmParameters::default()
            .with_crossover_prob(1.0)
            .with_random_seed(0);
        let graph_example_first = graph_with_single_node();
        let graph_example_second = graph_with_single_node();
        let requirements = GraphRequirements::default();
        let graph_gen_params = graph_generation_params();
        parameters.crossover_types = vec![crossover_type];
        let crossover = Crossover::new(parameters, requirements, graph_gen_params);
        let new_graphs = crossover.call(vec![
            Individual::new(Arc::new(graph_example_first.clone())),
            Individual::new(Arc::new(graph_example_second.clone())),
        ]);
        assert!(graphs_same(&new_graphs[0].graph, &graph_example_first));
        assert!(graphs_same(&new_graphs[1].graph, &graph_example_second));
    }
}
