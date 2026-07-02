use std::cell::RefCell;

use super::crossover::Crossover;
use super::mutation::Mutation;
use super::selection::Selection;
use super::{EvaluationOperator, PopulationT};
use crate::golem::optimisers::genetic::constants::{EVALUATION_ATTEMPTS_NUMBER, MIN_POP_SIZE};
use crate::golem::optimisers::genetic::params::GPAlgorithmParameters;

#[derive(Debug, Clone)]
pub struct EvaluationAttemptsError {
    pub message: String,
}

impl EvaluationAttemptsError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for EvaluationAttemptsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.message.is_empty() {
                "Too many fitness evaluation errors."
            } else {
                &self.message
            }
        )
    }
}

impl std::error::Error for EvaluationAttemptsError {}

pub struct ReproductionController {
    pub parameters: GPAlgorithmParameters,
    pub selection: Selection,
    pub mutation: Mutation,
    pub crossover: Crossover,
    minimum_valid_ratio: f64,
    success_rate_window: RefCell<Vec<f64>>,
}

impl ReproductionController {
    pub fn new(
        parameters: GPAlgorithmParameters,
        selection: Selection,
        mutation: Mutation,
        crossover: Crossover,
    ) -> Self {
        Self::with_window_size(parameters, selection, mutation, crossover, 10)
    }

    pub fn with_window_size(
        parameters: GPAlgorithmParameters,
        selection: Selection,
        mutation: Mutation,
        crossover: Crossover,
        window_size: usize,
    ) -> Self {
        Self {
            minimum_valid_ratio: parameters.required_valid_ratio * 0.5,
            success_rate_window: RefCell::new(vec![1.0; window_size]),
            parameters,
            selection,
            mutation,
            crossover,
        }
    }

    pub fn mean_success_rate(&self) -> f64 {
        let window = self.success_rate_window.borrow();
        if window.is_empty() {
            return 1.0;
        }
        window.iter().sum::<f64>() / window.len() as f64
    }

    pub fn minimum_valid_ratio(&self) -> f64 {
        self.minimum_valid_ratio
    }

    pub fn reproduce_uncontrolled(
        &self,
        population: PopulationT,
        evaluator: &EvaluationOperator,
        pop_size: Option<usize>,
    ) -> PopulationT {
        let selected = self.selection.call(population, pop_size);
        let crossed = self.crossover.call(selected);
        let mutated = self.mutation.call_population(crossed);
        evaluator(mutated)
    }

    pub fn reproduce(
        &self,
        population: PopulationT,
        evaluator: &EvaluationOperator,
    ) -> Result<PopulationT, EvaluationAttemptsError> {
        let total_target_size = self.parameters.pop_size;
        let mut collected_next_population = std::collections::HashMap::new();

        for _ in 0..EVALUATION_ATTEMPTS_NUMBER {
            let residual_size = total_target_size.saturating_sub(collected_next_population.len());
            let mut batch_size = (residual_size as f64 / self.mean_success_rate()).floor() as usize;
            batch_size = batch_size.max(MIN_POP_SIZE);
            batch_size = batch_size.min(population.len());

            let partial_next_population =
                self.reproduce_uncontrolled(population.clone(), evaluator, Some(batch_size));

            if partial_next_population.len() >= MIN_POP_SIZE {
                let valid_ratio = partial_next_population.len() as f64 / batch_size as f64;
                let mut window = self.success_rate_window.borrow_mut();
                if !window.is_empty() {
                    window.rotate_right(1);
                    window[0] = valid_ratio;
                }
            }

            for ind in partial_next_population {
                collected_next_population.insert(ind.uid.clone(), ind);
            }

            let required_threshold =
                total_target_size as f64 * self.parameters.required_valid_ratio;
            if collected_next_population.len() as f64 >= required_threshold {
                let mut result: Vec<_> = collected_next_population.into_values().collect();
                result.truncate(total_target_size);
                return Ok(result);
            }
        }

        let helpful_msg = "Check objective, constraints and evo operators. \
                           Possibly they return too few valid individuals.";
        let collected_len = collected_next_population.len();
        let minimum_threshold = total_target_size as f64 * self.minimum_valid_ratio;

        if collected_len as f64 >= minimum_threshold {
            return Ok(collected_next_population.into_values().collect());
        }

        Err(EvaluationAttemptsError::new(format!(
            "Could not collect valid individuals for next population.{helpful_msg}"
        )))
    }
}
