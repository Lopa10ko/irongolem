use std::sync::Arc;

use irongolem::golem::dag::{has_no_self_cycled_nodes, Graph};
use irongolem::golem::optimisers::genetic::operators::base_mutations::MutationTypesEnum;
use irongolem::golem::optimisers::genetic::params::{
    GPAlgorithmParameters, GraphGenerationParams, GraphRequirements,
};
use irongolem::golem::optimisers::genetic::EvoGraphOptimizer;
use irongolem::golem::optimisers::initial_population_generator::InitialPopulationGenerator;
use irongolem::golem::optimisers::objective::ObjectiveEvaluate;
use test_support::fixtures::{
    custom_initial_graphs, custom_objective, CustomDirectAdapter, CustomModel,
};

#[test]
fn test_custom_graph_opt() {
    let nodes_types = vec!["A", "B", "C", "D"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

    let mut requirements = GraphRequirements::default();
    requirements.num_of_generations = Some(5);
    requirements.show_progress = false;
    requirements.early_stopping_iterations = Some(1000);

    let optimiser_parameters = GPAlgorithmParameters::new(5)
        .with_mutation_types(vec![
            MutationTypesEnum::Simple,
            MutationTypesEnum::Reduce,
            MutationTypesEnum::Growth,
            MutationTypesEnum::LocalGrowth,
        ])
        .with_random_seed(1);

    let mut graph_generation_params = GraphGenerationParams::new(nodes_types);
    graph_generation_params.verifier = Arc::new(|graph| has_no_self_cycled_nodes(graph).is_ok());

    let adapter = CustomDirectAdapter;
    let objective = custom_objective();
    let initial_graphs: Vec<_> = custom_initial_graphs();
    let init_population = InitialPopulationGenerator::new(
        optimiser_parameters.pop_size,
        graph_generation_params.clone(),
        requirements.clone(),
    )
    .with_initial_graphs(
        initial_graphs
            .into_iter()
            .map(|m| adapter.adapt(m))
            .collect(),
    )
    .generate();

    let mut optimiser = EvoGraphOptimizer::new(
        objective.clone(),
        Some(init_population),
        requirements,
        graph_generation_params,
        optimiser_parameters,
    );

    let objective_eval = ObjectiveEvaluate::new(objective);
    let optimized_graphs = optimiser.optimise(&objective_eval).expect("optimise");
    let optimized_network = adapter.restore(optimized_graphs[0].clone());

    assert!(optimized_network.length() > 1);
    assert!(!optimized_network.nodes().is_empty());
}
