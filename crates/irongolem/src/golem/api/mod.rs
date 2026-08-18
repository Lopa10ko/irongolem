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
use crate::golem::optimisers::objective::Objective;

pub struct Golem {
    pub gp_algorithm_parameters: GPAlgorithmParameters,
    pub graph_generation_parameters: GraphGenerationParams,
    pub graph_requirements: GraphRequirements,
    objective: Objective,
    initial_graphs: Option<Vec<Arc<GraphDelegate>>>,
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
    ) -> Self {
        let requirements = GraphRequirements {
            timeout: timeout_minutes.map(|m| Duration::from_secs_f64(m * 60.0)),
            early_stopping_iterations,
            n_jobs,
            show_progress: false,
            ..GraphRequirements::default()
        };

        let gp_params = GPAlgorithmParameters {
            max_pop_size,
            mutation_types,
            crossover_types,
            ..GPAlgorithmParameters::default()
        };

        let graph_generation_parameters = GraphGenerationParams::new(available_node_types);

        Self {
            gp_algorithm_parameters: gp_params,
            graph_generation_parameters,
            graph_requirements: requirements,
            objective,
            initial_graphs: initial_graphs.map(|graphs| {
                let adapter = DirectAdapter;
                adapter.adapt_many(graphs)
            }),
        }
    }

    pub fn optimise(
        &mut self,
        objective_eval: &crate::golem::optimisers::objective::ObjectiveEvaluate,
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
        optimizer.optimise(objective_eval)
    }
}
