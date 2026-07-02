use std::collections::HashMap;

use crate::golem::optimisers::fitness::is_metric_worse;
use crate::golem::optimisers::genetic::operators::PopulationT;
use crate::golem::optimisers::objective::Objective;

pub trait ImprovementWatcher {
    fn is_any_improved(&self) -> bool;
    fn is_quality_improved(&self) -> bool;
    fn is_complexity_improved(&self) -> bool;
}

pub struct GenerationKeeper {
    objective: Option<Objective>,
    generation_num: usize,
    metrics_improvement: HashMap<String, bool>,
    archive_fitness: HashMap<String, Vec<f64>>,
}

impl GenerationKeeper {
    pub fn new(objective: Option<Objective>) -> Self {
        let mut keeper = Self {
            objective,
            generation_num: 0,
            metrics_improvement: HashMap::new(),
            archive_fitness: HashMap::new(),
        };
        keeper.reset_metrics_improvement();
        keeper
    }

    pub fn append(&mut self, population: &PopulationT) {
        let previous = self.archive_fitness.clone();
        self.update_archive_fitness(population);
        self.update_improvements(&previous);
        self.generation_num += 1;
    }

    fn metric_ids(&self) -> Vec<String> {
        self.objective
            .as_ref()
            .map(|o| o.metrics.keys().cloned().collect())
            .unwrap_or_default()
    }

    fn reset_metrics_improvement(&mut self) {
        self.metrics_improvement = self
            .metric_ids()
            .into_iter()
            .map(|id| (id, false))
            .collect();
    }

    fn update_archive_fitness(&mut self, population: &PopulationT) {
        if population.is_empty() {
            return;
        }
        let fitness_len = population[0].fitness.values().len().max(1);
        for (metric_idx, metric_id) in self.metric_ids().iter().enumerate() {
            let values: Vec<f64> = population
                .iter()
                .map(|ind| {
                    ind.fitness
                        .values()
                        .get(metric_idx)
                        .copied()
                        .unwrap_or(f64::INFINITY)
                })
                .collect();
            self.archive_fitness.insert(metric_id.clone(), values);
        }
        let _ = fitness_len;
    }

    fn update_improvements(&mut self, previous_metric_archive: &HashMap<String, Vec<f64>>) {
        self.reset_metrics_improvement();
        for metric in self.metric_ids() {
            let previous_worst = previous_metric_archive
                .get(&metric)
                .and_then(|v| v.iter().copied().reduce(f64::max))
                .unwrap_or(f64::INFINITY);
            let current_worst = self
                .archive_fitness
                .get(&metric)
                .and_then(|v| v.iter().copied().reduce(f64::max))
                .unwrap_or(f64::INFINITY);
            if is_metric_worse(&[previous_worst], &[current_worst]) {
                self.metrics_improvement.insert(metric, true);
            }
        }
    }
}

impl ImprovementWatcher for GenerationKeeper {
    fn is_any_improved(&self) -> bool {
        self.metrics_improvement.values().any(|&v| v)
    }

    fn is_quality_improved(&self) -> bool {
        self.objective
            .as_ref()
            .map(|o| {
                o.quality_metrics()
                    .iter()
                    .any(|id| self.metrics_improvement.get(id).copied().unwrap_or(false))
            })
            .unwrap_or(self.is_any_improved())
    }

    fn is_complexity_improved(&self) -> bool {
        self.objective
            .as_ref()
            .map(|o| {
                o.complexity_metrics()
                    .iter()
                    .any(|id| self.metrics_improvement.get(id).copied().unwrap_or(false))
            })
            .unwrap_or(false)
    }
}
