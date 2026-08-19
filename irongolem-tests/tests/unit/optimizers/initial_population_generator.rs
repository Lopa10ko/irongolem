use std::sync::Arc;

use irongolem::golem::adapter::DirectAdapter;
use irongolem::golem::dag::Graph;
use irongolem::golem::optimisers::genetic::params::{
    GPAlgorithmParameters, GraphGenerationParams, GraphRequirements,
};
use irongolem::golem::optimisers::genetic::EvoGraphOptimizer;
use irongolem::golem::optimisers::initial_population_generator::InitialPopulationGenerator;
use irongolem::golem::optimisers::objective::Objective;
use test_support::fixtures::{graph_first, graph_second, graph_third};

fn setup_test(
    pop_size: usize,
) -> (
    GraphRequirements,
    GraphGenerationParams,
    InitialPopulationGenerator,
) {
    let requirements = GraphRequirements::default();
    let graph_generation_params = GraphGenerationParams::new(
        vec!["a", "b", "c", "d", "e", "f"]
            .into_iter()
            .map(String::from)
            .collect(),
    );
    let generator = InitialPopulationGenerator::new(
        pop_size,
        graph_generation_params.clone(),
        requirements.clone(),
    );
    (requirements, graph_generation_params, generator)
}

#[test]
fn test_random_initial_population() {
    let (requirements, graph_generation_params, mut generator) = setup_test(3);
    let generated_population = generator.generate();
    let max_depth = requirements.max_depth;
    let verifier = &graph_generation_params.verifier;
    assert_eq!(generated_population.len(), 3);
    assert!(generated_population
        .iter()
        .all(|g| g.as_ref().depth() <= max_depth && verifier(g)));
}

#[test]
fn test_initial_graphs_as_initial_population() {
    let adapter = DirectAdapter;
    let initial_graphs = adapter.adapt_many(vec![graph_first(), graph_second(), graph_third()]);

    let (_, _, mut generator) = setup_test(3);
    generator = generator.with_initial_graphs(initial_graphs.clone());
    let generated = generator.generate();
    assert_eq!(generated.len(), initial_graphs.len());
    for g in &generated {
        assert!(initial_graphs.iter().any(|ig| Arc::ptr_eq(ig, g)));
    }

    let (_, _, mut generator) = setup_test(4);
    generator = generator.with_initial_graphs(initial_graphs.clone());
    let generated = generator.generate();
    assert_eq!(generated.len(), initial_graphs.len());

    let (_, _, mut generator) = setup_test(2);
    generator = generator.with_initial_graphs(initial_graphs.clone());
    let generated = generator.generate();
    assert_eq!(generated.len(), 2);
    assert!(generated
        .iter()
        .all(|g| initial_graphs.iter().any(|ig| Arc::ptr_eq(ig, g))));
}

#[test]
fn test_initial_population_generation_function() {
    for pop_size in [3usize, 4] {
        let (_, graph_generation_params, mut generator) = setup_test(pop_size);
        let choices = vec![graph_first(), graph_second(), graph_third()];
        let mut idx = 0usize;
        generator = generator.with_custom_generation_function(Box::new(move || {
            let g = choices[idx % choices.len()].clone();
            idx += 1;
            g
        }));
        let verifier = &graph_generation_params.verifier;
        let generated = generator.generate();
        assert!(generated.len() <= 3);
        assert!(generated.iter().all(|g| verifier(g)));
        let unique_len = generated
            .iter()
            .filter(|g| {
                generated
                    .iter()
                    .filter(|other| other.as_ref() == g.as_ref())
                    .count()
                    == 1
            })
            .count();
        assert_eq!(unique_len, generated.len());
    }
}

#[test]
fn test_evo_optimizer_generates_population_when_graphs_absent() {
    let pop_size = 3;
    let (requirements, graph_generation_params, _) = setup_test(pop_size);
    let params = GPAlgorithmParameters::new(pop_size);
    let objective = Objective::new(std::collections::HashMap::new());

    for initial_graphs in [None, Some(Vec::new())] {
        let opt = EvoGraphOptimizer::new(
            objective.clone(),
            initial_graphs,
            requirements.clone(),
            graph_generation_params.clone(),
            params.clone(),
        );
        let generated = opt
            .populational
            .base
            .initial_graphs
            .as_ref()
            .expect("generated population should be stored");
        assert_eq!(generated.len(), pop_size);
        let verifier = &graph_generation_params.verifier;
        assert!(generated.iter().all(|g| verifier(g)));
    }
}

#[test]
fn test_evo_optimizer_adapts_and_verifies_supplied_graphs() {
    let adapter = DirectAdapter;
    let initial_graphs = adapter.adapt_many(vec![graph_first(), graph_second(), graph_third()]);
    let (requirements, graph_generation_params, _) = setup_test(5);
    let opt = EvoGraphOptimizer::new(
        Objective::new(std::collections::HashMap::new()),
        Some(initial_graphs.clone()),
        requirements,
        graph_generation_params.clone(),
        GPAlgorithmParameters::new(5),
    );
    let stored = opt
        .populational
        .base
        .initial_graphs
        .as_ref()
        .expect("supplied graphs should be stored");
    assert_eq!(stored.len(), initial_graphs.len());
    let verifier = &graph_generation_params.verifier;
    assert!(stored.iter().all(|g| verifier(g)));
}
