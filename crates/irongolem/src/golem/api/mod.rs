use std::sync::Arc;
use std::time::Duration;

use crate::golem::adapter::DirectAdapter;
use crate::golem::dag::GraphDelegate;
use crate::golem::optimisers::genetic::operators::{
    base_mutations::MutationTypesEnum, crossover::CrossoverTypesEnum,
};
use crate::golem::optimisers::genetic::params::{
    GPAlgorithmParameters, GraphGenerationParams, GraphRequirements,
};
use crate::golem::optimisers::genetic::EvoGraphOptimizer;
use crate::golem::optimisers::objective::{Objective, ObjectiveEvaluate};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GolemConfigError {
    pub message: String,
}

impl GolemConfigError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for GolemConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GolemConfigError {}

pub struct Golem {
    pub gp_algorithm_parameters: GPAlgorithmParameters,
    pub graph_generation_parameters: GraphGenerationParams,
    pub graph_requirements: GraphRequirements,
    objective: Objective,
    initial_graphs: Option<Vec<Arc<GraphDelegate>>>,
}

fn timeout_from_minutes(
    timeout_minutes: Option<f64>,
) -> Result<Option<Duration>, GolemConfigError> {
    let Some(minutes) = timeout_minutes else {
        return Ok(None);
    };
    if !minutes.is_finite() || minutes < 0.0 {
        return Err(GolemConfigError::new(format!(
            "timeout_minutes must be a finite non-negative number, got {minutes}"
        )));
    }
    let seconds = minutes * 60.0;
    Duration::try_from_secs_f64(seconds).map(Some).map_err(|_| {
        GolemConfigError::new(format!(
            "timeout_minutes {minutes} overflows Duration when converted to seconds"
        ))
    })
}

impl Golem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        timeout_minutes: Option<f64>,
        early_stopping_iterations: Option<usize>,
        n_jobs: i32,
        initial_graphs: Option<Vec<GraphDelegate>>,
        objective: Objective,
        max_pop_size: Option<usize>,
        mutation_types: Vec<MutationTypesEnum>,
        crossover_types: Vec<CrossoverTypesEnum>,
        available_node_types: Vec<String>,
    ) -> Result<Self, GolemConfigError> {
        let requirements = GraphRequirements {
            timeout: timeout_from_minutes(timeout_minutes)?,
            early_stopping_iterations,
            n_jobs,
            show_progress: false,
            ..GraphRequirements::default()
        };

        let gp_params = GPAlgorithmParameters {
            max_pop_size,
            mutation_types,
            crossover_types,
            multi_objective: objective.is_multi_objective(),
            ..GPAlgorithmParameters::default()
        };

        let graph_generation_parameters = GraphGenerationParams::new(available_node_types);

        Ok(Self {
            gp_algorithm_parameters: gp_params,
            graph_generation_parameters,
            graph_requirements: requirements,
            objective,
            initial_graphs: initial_graphs.map(|graphs| {
                let adapter = DirectAdapter;
                adapter.adapt_many(graphs)
            }),
        })
    }

    pub fn optimise(
        &mut self,
    ) -> Result<
        Vec<Arc<GraphDelegate>>,
        crate::golem::optimisers::genetic::operators::reproduction::EvaluationAttemptsError,
    > {
        let mut optimizer = EvoGraphOptimizer::new(
            self.objective.clone(),
            self.initial_graphs.clone(),
            self.graph_requirements.clone(),
            self.graph_generation_parameters.clone(),
            self.gp_algorithm_parameters.clone(),
        );
        let objective_eval = ObjectiveEvaluate::new(self.objective.clone());
        optimizer.optimise(&objective_eval)
    }
}
