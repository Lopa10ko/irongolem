use std::time::Duration;

use irongolem::golem::api::Golem;
use irongolem::golem::optimisers::genetic::operators::{
    base_mutations::MutationTypesEnum, crossover::CrossoverTypesEnum,
};
use irongolem::golem::optimisers::genetic::params::GPAlgorithmParameters;
use irongolem::golem::optimisers::objective::Objective;

fn sample_golem(
    timeout_minutes: Option<f64>,
) -> Result<Golem, irongolem::golem::api::GolemConfigError> {
    Golem::new(
        timeout_minutes,
        Some(100),
        1,
        None,
        Objective::new(std::collections::HashMap::new()),
        Some(50),
        vec![
            MutationTypesEnum::SingleAdd,
            MutationTypesEnum::SingleDrop,
            MutationTypesEnum::SingleChange,
        ],
        vec![CrossoverTypesEnum::Subtree],
        vec!["a".into(), "b".into()],
    )
}

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
    )
    .expect("valid timeout");

    let expected_gp = GPAlgorithmParameters::default()
        .with_mutation_types(vec![
            MutationTypesEnum::SingleAdd,
            MutationTypesEnum::SingleDrop,
            MutationTypesEnum::SingleChange,
        ])
        .with_crossover_types(vec![CrossoverTypesEnum::Subtree]);

    assert_eq!(golem.gp_algorithm_parameters.pop_size, expected_gp.pop_size);
    assert!(!golem.gp_algorithm_parameters.multi_objective);
    assert_eq!(
        golem.graph_generation_parameters.available_node_types,
        node_types
    );
    assert_eq!(golem.graph_requirements.n_jobs, 1);
    assert_eq!(
        golem.graph_requirements.early_stopping_iterations,
        Some(100)
    );
    assert_eq!(
        golem.graph_requirements.timeout,
        Some(Duration::from_secs(60))
    );
}

#[test]
fn test_api_multi_objective_from_objective() {
    let golem = Golem::new(
        Some(1.0),
        Some(100),
        1,
        None,
        Objective::multi_objective(
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
        ),
        Some(50),
        vec![MutationTypesEnum::SingleAdd],
        vec![CrossoverTypesEnum::Subtree],
        vec!["a".into()],
    )
    .expect("valid timeout");

    assert!(golem.gp_algorithm_parameters.multi_objective);
}

#[test]
fn test_api_timeout_none_and_invalid() {
    let golem = sample_golem(None).expect("no timeout is valid");
    assert_eq!(golem.graph_requirements.timeout, None);

    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0, f64::MAX] {
        assert!(
            sample_golem(Some(invalid)).is_err(),
            "expected error for timeout_minutes={invalid}"
        );
    }
}
