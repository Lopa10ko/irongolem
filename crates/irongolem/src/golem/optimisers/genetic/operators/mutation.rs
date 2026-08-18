use std::sync::Arc;

use super::base_mutations::{mutation_fn, MutationTypesEnum};
use super::operator::{OperatorBase, PopulationT};
use crate::golem::optimisers::genetic::params::{GPAlgorithmParameters, GraphGenerationParams};
use crate::golem::optimisers::genetic::GraphRequirements;
use crate::golem::optimisers::history::Individual;

#[derive(Debug, Clone)]
pub struct Mutation {
    base: OperatorBase,
    graph_generation_params: GraphGenerationParams,
}

impl Mutation {
    pub fn new(
        parameters: GPAlgorithmParameters,
        requirements: GraphRequirements,
        graph_gen_params: GraphGenerationParams,
    ) -> Self {
        Self {
            base: OperatorBase::new(parameters, requirements),
            graph_generation_params: graph_gen_params,
        }
    }

    /// Apply mutation to a single individual or a population.
    pub fn call(&self, target: MutationTarget) -> MutationResult {
        match target {
            MutationTarget::Individual(individual) => {
                MutationResult::Individual(self.mutate_individual(individual))
            }
            MutationTarget::Population(population) => {
                MutationResult::Population(self.mutate_population(population))
            }
        }
    }

    /// Convenience wrapper used by reproduction controller.
    pub fn call_population(&self, population: PopulationT) -> PopulationT {
        self.mutate_population(population)
    }

    fn mutate_population(&self, population: PopulationT) -> PopulationT {
        population
            .into_iter()
            .filter_map(|ind| self.mutate_individual(ind))
            .collect()
    }

    fn mutate_individual(&self, individual: Individual) -> Option<Individual> {
        let rng = &self.base.rng;
        let mutation_type = rng
            .random_choice(&self.base.parameters.mutation_types)
            .unwrap_or(MutationTypesEnum::SingleChange);
        let is_applied = self.will_mutation_be_applied(mutation_type);
        if !is_applied {
            return Some(individual);
        }

        let init_graph = individual.graph.clone();
        let parent = individual;
        let mut result = parent.clone();

        for _ in 0..self.base.parameters.max_num_of_operator_attempts {
            let new_graph = result.graph.deep_clone();
            let mutation_func = mutation_fn(mutation_type);
            let new_graph = mutation_func(
                new_graph,
                &self.base.requirements,
                &self.graph_generation_params,
                &self.base.parameters,
                rng,
            );

            if (self.graph_generation_params.verifier)(&new_graph) {
                result = Individual::new(Arc::new(new_graph));
                break;
            }
        }

        if *result.graph == *init_graph {
            None
        } else {
            let op = crate::golem::optimisers::history::ParentOperator::new(
                "mutation",
                format!("{mutation_type:?}"),
                vec![parent],
            );
            Some(result.with_parent_operator(op))
        }
    }

    fn will_mutation_be_applied(&self, mutation_type: MutationTypesEnum) -> bool {
        self.base.rng.gen_f64() <= self.base.parameters.mutation_prob
            && mutation_type != MutationTypesEnum::None
    }
}

#[allow(clippy::large_enum_variant)]
pub enum MutationTarget {
    Individual(Individual),
    Population(PopulationT),
}

#[allow(clippy::large_enum_variant)]
pub enum MutationResult {
    Individual(Option<Individual>),
    Population(PopulationT),
}
