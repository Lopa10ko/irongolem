use std::collections::HashMap;

use super::operators::EvaluationOperator;
use super::operators::PopulationT;
use crate::golem::dag::Graph;
use crate::golem::optimisers::genetic::constants::MIN_POP_SIZE;
use crate::golem::optimisers::genetic::rng::GeneticRng;
use crate::golem::optimisers::history::Individual;

/// Increases structural uniqueness of `population`, refilling to [`MIN_POP_SIZE`] when needed.
pub fn get_structure_unique_population(
    population: PopulationT,
    evaluator: EvaluationOperator,
    rng: &GeneticRng,
) -> PopulationT {
    let mut unique_population_with_ids: HashMap<String, Individual> = HashMap::new();
    for ind in population {
        unique_population_with_ids
            .entry(ind.graph.as_ref().descriptive_id())
            .or_insert(ind);
    }
    let mut unique_population: Vec<_> = unique_population_with_ids.into_values().collect();

    if unique_population.len() < MIN_POP_SIZE {
        unique_population = extend_population(unique_population, MIN_POP_SIZE, rng);
    }

    evaluator(unique_population)
}

fn extend_population(
    mut pop: PopulationT,
    target_pop_size: usize,
    rng: &GeneticRng,
) -> PopulationT {
    if pop.is_empty() {
        return pop;
    }
    let n = target_pop_size.saturating_sub(pop.len());
    for _ in 0..n {
        let idx = rng.gen_range(0..pop.len());
        let template = pop[idx].clone();
        pop.push(Individual::with_uid(
            uuid::Uuid::new_v4().to_string(),
            template.graph.clone(),
        ));
    }
    pop
}
