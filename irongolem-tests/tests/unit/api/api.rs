use irongolem::golem::api::Golem;
use irongolem::golem::optimisers::genetic::operators::{
    base_mutations::MutationTypesEnum, crossover::CrossoverTypesEnum,
};
use irongolem::golem::optimisers::genetic::params::GPAlgorithmParameters;
use irongolem::golem::optimisers::objective::Objective;

#[test]
fn test_specifying_parameters_through_api() {
    let node_types = vec!["a".into(), "b".into()];
    let objective = Objective::new(std::collections::HashMap::new());

    let golem = Golem::new(
        Some(1.0),
        Some(100),
        1,
        None,
        objective,
        Some(50),
        vec![
            MutationTypesEnum::SingleAdd,
            MutationTypesEnum::SingleDrop,
            MutationTypesEnum::SingleChange,
        ],
        vec![CrossoverTypesEnum::Subtree],
        node_types.clone(),
    );

    let expected_gp = GPAlgorithmParameters::default()
        .with_mutation_types(vec![
            MutationTypesEnum::SingleAdd,
            MutationTypesEnum::SingleDrop,
            MutationTypesEnum::SingleChange,
        ])
        .with_crossover_types(vec![CrossoverTypesEnum::Subtree]);

    assert_eq!(golem.gp_algorithm_parameters.pop_size, expected_gp.pop_size);
    assert_eq!(
        golem.graph_generation_parameters.available_node_types,
        node_types
    );
    assert_eq!(golem.graph_requirements.n_jobs, 1);
    assert_eq!(
        golem.graph_requirements.early_stopping_iterations,
        Some(100)
    );
}
