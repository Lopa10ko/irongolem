//! operators_reproducibility

use std::sync::Arc;

use irongolem::golem::dag::Graph;
use irongolem::golem::optimisers::genetic::operators::base_mutations::MutationTypesEnum;
use irongolem::golem::optimisers::genetic::operators::crossover::{Crossover, CrossoverTypesEnum};
use irongolem::golem::optimisers::genetic::operators::mutation::{
    Mutation, MutationResult, MutationTarget,
};
use irongolem::golem::optimisers::genetic::params::{
    GPAlgorithmParameters, GraphGenerationParams, GraphRequirements,
};
use irongolem::golem::optimisers::genetic::rng::GeneticRng;
use irongolem::golem::optimisers::history::Individual;
use test_support::fixtures::{get_mutation_params, graph_first, graph_second, MutationParams};

const CROSSOVER_TYPES: [CrossoverTypesEnum; 5] = [
    CrossoverTypesEnum::Subtree,
    CrossoverTypesEnum::OnePoint,
    CrossoverTypesEnum::ExchangeEdges,
    CrossoverTypesEnum::ExchangeParentsOne,
    CrossoverTypesEnum::ExchangeParentsBoth,
];

const SEEDS: [u64; 3] = [0, 42, 1042];

const MUTATION_TYPES: [MutationTypesEnum; 10] = [
    MutationTypesEnum::Simple,
    MutationTypesEnum::Growth,
    MutationTypesEnum::LocalGrowth,
    MutationTypesEnum::TreeGrowth,
    MutationTypesEnum::Reduce,
    MutationTypesEnum::SingleAdd,
    MutationTypesEnum::SingleChange,
    MutationTypesEnum::SingleDrop,
    MutationTypesEnum::SingleEdge,
    MutationTypesEnum::None,
];

#[test]
fn test_crossover_reproducibility() {
    let graph_example_first = graph_first();
    let graph_example_second = graph_second();
    let requirements = GraphRequirements::default();
    let graph_generation_params = GraphGenerationParams::new(
        vec!["a", "b", "c", "d"]
            .into_iter()
            .map(String::from)
            .collect(),
    );

    for crossover_type in CROSSOVER_TYPES {
        for seed in SEEDS {
            let run_with_seed = |seed: u64| {
                let mut parameters =
                    GPAlgorithmParameters::default().with_random_seed(seed);
                parameters.crossover_types = vec![crossover_type];
                let crossover = Crossover::new(
                    parameters,
                    requirements.clone(),
                    graph_generation_params.clone(),
                );
                let results = crossover.call(vec![
                    Individual::new(Arc::new(graph_example_first.clone())),
                    Individual::new(Arc::new(graph_example_second.clone())),
                ]);
                results
                    .iter()
                    .map(|ind| ind.graph.descriptive_id())
                    .collect::<Vec<_>>()
            };

            let results_first = run_with_seed(seed);
            let results_second = run_with_seed(seed);
            assert_eq!(results_first, results_second);
        }
    }
}

#[test]
fn test_mutation_reproducibility() {
    for mutation_type in MUTATION_TYPES {
        for seed in SEEDS {
            let run_with_seed = |seed: u64| {
                let MutationParams {
                    requirements,
                    graph_gen_params,
                    parameters,
                } = get_mutation_params(Some(vec![mutation_type]), None, 1.0);
                let parameters = parameters.with_random_seed(seed);
                let graph_gen_params = graph_gen_params.with_rng(GeneticRng::seeded(seed));
                let mutation = Mutation::new(parameters, requirements, graph_gen_params);
                let ind = Individual::new(Arc::new(graph_first()));
                match mutation.call(MutationTarget::Individual(ind)) {
                    MutationResult::Individual(Some(new_ind)) => new_ind.graph.descriptive_id(),
                    MutationResult::Individual(None) => String::new(),
                    _ => panic!("expected individual mutation result"),
                }
            };

            let results_first = run_with_seed(seed);
            let results_second = run_with_seed(seed);
            assert_eq!(results_first, results_second);
        }
    }
}
