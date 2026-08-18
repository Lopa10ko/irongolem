use std::sync::Arc;

use super::evaluation::{EvaluationDispatcher, MultiprocessingDispatcher, SequentialDispatcher};
use super::genetic::operators::reproduction::EvaluationAttemptsError;
use super::genetic::operators::PopulationT;
use super::genetic::parameters::generation_keeper::GenerationKeeper;
use super::genetic::params::{GPAlgorithmParameters, GraphGenerationParams, GraphRequirements};
use super::objective::{Objective, ObjectiveEvaluate};
use super::opt_history::OptHistory;
use super::timer::OptimisationTimer;
use crate::golem::dag::GraphDelegate;

pub struct GraphOptimizer {
    pub objective: Objective,
    pub initial_graphs: Option<Vec<Arc<GraphDelegate>>>,
    pub requirements: GraphRequirements,
    pub graph_generation_params: GraphGenerationParams,
    pub graph_optimizer_params: GPAlgorithmParameters,
    pub history: Option<OptHistory>,
}

impl GraphOptimizer {
    pub fn new(
        objective: Objective,
        initial_graphs: Option<Vec<Arc<GraphDelegate>>>,
        requirements: GraphRequirements,
        graph_generation_params: GraphGenerationParams,
        graph_optimizer_params: GPAlgorithmParameters,
    ) -> Self {
        let adapted_graphs = initial_graphs.map(|graphs| {
            graph_generation_params
                .adapter
                .adapt_many(graphs.into_iter().map(|g| (*g).clone()).collect())
                .into_iter()
                .filter(|g| (graph_generation_params.verifier)(g))
                .collect()
        });
        let history = if requirements.keep_history {
            Some(OptHistory::new(Some(objective.get_info())))
        } else {
            None
        };
        Self {
            objective,
            initial_graphs: adapted_graphs,
            requirements,
            graph_generation_params,
            graph_optimizer_params,
            history,
        }
    }
}

pub trait PopulationalOptimizerTrait {
    fn optimise(
        &mut self,
        objective_eval: &ObjectiveEvaluate,
    ) -> Result<Vec<Arc<GraphDelegate>>, EvaluationAttemptsError>;

    fn current_generation_num(&self) -> usize;
    fn best_individuals(&self) -> Vec<super::history::Individual>;
}

pub struct PopulationalOptimizer {
    pub base: GraphOptimizer,
    pub population: Option<PopulationT>,
    pub generations: GenerationKeeper,
    pub timer: OptimisationTimer,
}

impl PopulationalOptimizer {
    pub fn new(
        objective: Objective,
        initial_graphs: Option<Vec<Arc<GraphDelegate>>>,
        requirements: GraphRequirements,
        graph_generation_params: GraphGenerationParams,
        graph_optimizer_params: GPAlgorithmParameters,
    ) -> Self {
        let timer = requirements
            .timeout
            .map(OptimisationTimer::new)
            .unwrap_or_else(OptimisationTimer::forever);
        let generations =
            GenerationKeeper::with_keep_n_best(Some(objective.clone()), requirements.keep_n_best);
        Self {
            base: GraphOptimizer::new(
                objective,
                initial_graphs,
                requirements,
                graph_generation_params,
                graph_optimizer_params,
            ),
            population: None,
            generations,
            timer,
        }
    }

    pub fn eval_dispatcher(&self) -> Box<dyn EvaluationDispatcher> {
        let adapter = self.base.graph_generation_params.adapter.clone();
        if self.base.requirements.n_jobs == 1 {
            Box::new(SequentialDispatcher::new(adapter))
        } else {
            Box::new(MultiprocessingDispatcher::with_n_jobs(
                adapter,
                self.base.requirements.n_jobs,
            ))
        }
    }

    pub fn best_individuals(&self) -> Vec<super::history::Individual> {
        self.generations.best_individuals()
    }

    pub fn should_stop(&mut self) -> bool {
        let req = &self.base.requirements;
        if self
            .timer
            .is_time_limit_reached(Some(self.current_generation_num().saturating_sub(1)))
        {
            return true;
        }
        if let Some(max_gens) = req.num_of_generations {
            if self.current_generation_num() > max_gens {
                return true;
            }
        }
        let max_stagnation = req
            .early_stopping_iterations
            .unwrap_or(req.num_of_generations.unwrap_or(usize::MAX));
        if self.generations.stagnation_iter_count() >= max_stagnation {
            return true;
        }
        let max_stagnation_time = req.early_stopping_timeout.unwrap_or(f64::MAX);
        if self.generations.stagnation_time_duration() >= max_stagnation_time {
            return true;
        }
        false
    }

    pub fn update_population(
        &mut self,
        next_population: PopulationT,
        label: Option<&str>,
        metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    ) {
        self.generations.append(&next_population);
        if self.base.requirements.keep_history {
            if let Some(ref mut history) = self.base.history {
                history.add_to_history(next_population.clone(), label, metadata);
                history.add_to_archive_history(&self.generations.best_individuals());
            }
        }
        self.population = Some(next_population);
    }
}

impl PopulationalOptimizerTrait for PopulationalOptimizer {
    fn current_generation_num(&self) -> usize {
        self.generations.generation_num()
    }

    fn best_individuals(&self) -> Vec<super::history::Individual> {
        self.generations.best_individuals()
    }

    fn optimise(
        &mut self,
        _objective_eval: &ObjectiveEvaluate,
    ) -> Result<Vec<Arc<GraphDelegate>>, EvaluationAttemptsError> {
        Err(EvaluationAttemptsError::new("not implemented in base"))
    }
}
