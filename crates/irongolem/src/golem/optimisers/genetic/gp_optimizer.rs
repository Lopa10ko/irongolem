use std::sync::Arc;

use super::constants::MAX_GRAPH_GEN_ATTEMPTS;
use super::operators::crossover::Crossover;
use super::operators::elitism::Elitism;
use super::operators::mutation::{Mutation, MutationResult, MutationTarget};
use super::operators::reproduction::{EvaluationAttemptsError, ReproductionController};
use super::operators::selection::Selection;
use super::operators::{EvaluationOperator, PopulationT};
use super::params::{GPAlgorithmParameters, GraphGenerationParams, GraphRequirements};
use super::rng::random_choice;
use crate::golem::dag::{Graph, GraphDelegate};
use crate::golem::optimisers::evaluation::ObjectiveFn;
use crate::golem::optimisers::history::Individual;
use crate::golem::optimisers::objective::{Objective, ObjectiveEvaluate};
use crate::golem::optimisers::opt_history::OptHistory;
use crate::golem::optimisers::populational_optimizer::PopulationalOptimizer;

pub struct EvoGraphOptimizer {
    pub populational: PopulationalOptimizer,
    mutation: Mutation,
    elitism: Elitism,
    reproducer: ReproductionController,
    initial_individuals: Vec<Individual>,
}

impl EvoGraphOptimizer {
    pub fn new(
        objective: Objective,
        initial_graphs: Option<Vec<Arc<GraphDelegate>>>,
        requirements: GraphRequirements,
        graph_generation_params: GraphGenerationParams,
        graph_optimizer_params: GPAlgorithmParameters,
    ) -> Self {
        let mut requirements = requirements;
        if requirements.start_depth > 0 {
            requirements.max_depth = requirements.start_depth;
        }

        let adapted: Vec<Individual> = initial_graphs
            .unwrap_or_default()
            .into_iter()
            .map(Individual::new)
            .collect();

        let selection = Selection::new(graph_optimizer_params.clone(), requirements.clone());
        let crossover = Crossover::new(
            graph_optimizer_params.clone(),
            requirements.clone(),
            graph_generation_params.clone(),
        );
        let mutation = Mutation::new(
            graph_optimizer_params.clone(),
            requirements.clone(),
            graph_generation_params.clone(),
        );
        let elitism = Elitism::new(graph_optimizer_params.clone(), requirements.clone());
        let reproducer = ReproductionController::new(
            graph_optimizer_params.clone(),
            selection.clone(),
            mutation.clone(),
            crossover.clone(),
        );

        Self {
            populational: PopulationalOptimizer::new(
                objective,
                None,
                requirements,
                graph_generation_params,
                graph_optimizer_params,
            ),
            mutation,
            elitism,
            reproducer,
            initial_individuals: adapted,
        }
    }

    pub fn history(&self) -> Option<&OptHistory> {
        self.populational.base.history.as_ref()
    }

    pub fn graph_generation_params(&self) -> &GraphGenerationParams {
        &self.populational.base.graph_generation_params
    }

    pub fn optimise(
        &mut self,
        objective_eval: &ObjectiveEvaluate,
    ) -> Result<Vec<Arc<GraphDelegate>>, EvaluationAttemptsError> {
        let objective_fn: ObjectiveFn = Arc::new({
            let objective = objective_eval.objective.clone();
            move |graph| objective.evaluate(graph)
        });
        let evaluator = self
            .populational
            .eval_dispatcher()
            .dispatch(objective_fn, None);

        self.populational.timer.start();
        self.initial_population(&evaluator)?;

        while !self.populational.should_stop() {
            match self.evolve_population(&evaluator) {
                Ok(new_population) => {
                    self.populational
                        .update_population(new_population, None, None);
                }
                Err(ex) => {
                    let _ = ex;
                    break;
                }
            }
        }

        let best = self.populational.best_individuals();
        self.populational
            .update_population(best.clone(), Some("final_choices"), None);

        Ok(best.into_iter().map(|ind| ind.graph).collect())
    }

    fn initial_population(
        &mut self,
        evaluator: &EvaluationOperator,
    ) -> Result<(), EvaluationAttemptsError> {
        let evaluated = evaluator(self.initial_individuals.clone());
        self.populational
            .update_population(evaluated, Some("initial_assumptions"), None);

        let pop_size = self.populational.base.graph_optimizer_params.pop_size;
        if self.initial_individuals.len() < pop_size {
            if let Some(pop) = self.populational.population.clone() {
                let extended = self.extend_population(&pop, pop_size);
                let evaluated = evaluator(extended);
                self.populational.update_population(
                    evaluated,
                    Some("extended_initial_assumptions"),
                    None,
                );
            }
        }
        Ok(())
    }

    fn extend_population(&self, pop: &PopulationT, target: usize) -> PopulationT {
        let verifier = self
            .populational
            .base
            .graph_generation_params
            .verifier
            .clone();
        let mut extended = pop.clone();
        let mut pop_graph_ids: Vec<String> = extended
            .iter()
            .map(|i| i.graph.as_ref().descriptive_id())
            .collect();

        for _ in 0..MAX_GRAPH_GEN_ATTEMPTS {
            if extended.len() >= target {
                break;
            }
            let Some(parent) = random_choice(pop) else {
                break;
            };
            if let MutationResult::Individual(Some(new_ind)) = self
                .mutation
                .call(MutationTarget::Individual(parent.clone()))
            {
                let desc = new_ind.graph.as_ref().descriptive_id();
                if !pop_graph_ids.contains(&desc) && verifier(&new_ind.graph) {
                    pop_graph_ids.push(desc);
                    extended.push(new_ind);
                }
            }
        }
        extended
    }

    fn evolve_population(
        &self,
        evaluator: &EvaluationOperator,
    ) -> Result<PopulationT, EvaluationAttemptsError> {
        let population = self
            .populational
            .population
            .as_ref()
            .cloned()
            .unwrap_or_default();
        let new_population = self.reproducer.reproduce(population, evaluator)?;
        let best = self.populational.best_individuals();
        Ok(self.elitism.call(best, new_population))
    }
}
