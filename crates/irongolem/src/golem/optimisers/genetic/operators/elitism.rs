use std::cmp::Ordering;

use super::operator::{OperatorBase, PopulationT};
use crate::golem::optimisers::genetic::params::GPAlgorithmParameters;
use crate::golem::optimisers::genetic::GraphRequirements;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElitismTypesEnum {
    KeepNBest,
    ReplaceWorst,
    None,
}

#[derive(Debug, Clone)]
pub struct Elitism {
    base: OperatorBase,
}

impl Elitism {
    pub fn new(parameters: GPAlgorithmParameters, requirements: GraphRequirements) -> Self {
        Self {
            base: OperatorBase::new(parameters, requirements),
        }
    }

    pub fn call(&self, best_individuals: PopulationT, new_population: PopulationT) -> PopulationT {
        let elitism_type = self.base.parameters.elitism_type;
        if elitism_type == ElitismTypesEnum::None || !self.is_elitism_applicable() {
            return new_population;
        }
        match elitism_type {
            ElitismTypesEnum::KeepNBest => {
                keep_n_best_elitism(best_individuals, new_population, &self.base.rng)
            }
            ElitismTypesEnum::ReplaceWorst => {
                replace_worst_elitism(best_individuals, new_population)
            }
            ElitismTypesEnum::None => new_population,
        }
    }

    fn is_elitism_applicable(&self) -> bool {
        if self.base.parameters.multi_objective {
            return false;
        }
        self.base.parameters.pop_size >= self.base.parameters.min_pop_size_with_elitism
    }
}

pub fn keep_n_best_elitism(
    best_individuals: PopulationT,
    new_population: PopulationT,
    rng: &crate::golem::optimisers::genetic::rng::GeneticRng,
) -> PopulationT {
    let target_len = new_population.len();
    let mut final_population = best_individuals;
    let mut new_unique: Vec<_> = new_population
        .into_iter()
        .filter(|ind| !final_population.iter().any(|b| b.uid == ind.uid))
        .collect();
    if !new_unique.is_empty() {
        rng.shuffle(&mut new_unique);
        let remain_n = target_len.saturating_sub(final_population.len());
        final_population.extend(new_unique.into_iter().take(remain_n));
    }
    final_population
}

pub fn replace_worst_elitism(
    best_individuals: PopulationT,
    new_population: PopulationT,
) -> PopulationT {
    let target_len = new_population.len();
    let mut population = best_individuals;
    population.extend(new_population);
    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(Ordering::Equal));
    population.truncate(target_len);
    population
}
