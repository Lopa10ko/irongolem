use std::collections::HashMap;
use std::time::Instant;

use crate::golem::optimisers::archive::{HallOfFame, ParetoFront};
use crate::golem::optimisers::fitness::is_metric_worse;
use crate::golem::optimisers::genetic::operators::PopulationT;
use crate::golem::optimisers::objective::Objective;

const PARETO_MAX_POP_SIZE_MULTIPLIER: usize = 5;

pub trait ImprovementWatcher {
    fn stagnation_iter_count(&self) -> usize;
    fn stagnation_time_duration(&self) -> f64;
    fn is_any_improved(&self) -> bool;
    fn is_quality_improved(&self) -> bool;
    fn is_complexity_improved(&self) -> bool;
}

enum Archive {
    Hall(HallOfFame),
    Pareto(ParetoFront),
}

impl Archive {
    fn update(&mut self, population: &PopulationT) {
        match self {
            Archive::Hall(h) => h.update(population),
            Archive::Pareto(p) => p.update(population),
        }
    }

    fn items(&self) -> Vec<crate::golem::optimisers::history::Individual> {
        match self {
            Archive::Hall(h) => h.items.clone(),
            Archive::Pareto(p) => p.items.clone(),
        }
    }
}

pub struct GenerationKeeper {
    objective: Option<Objective>,
    generation_num: usize,
    stagnation_counter: usize,
    stagnation_start: Instant,
    metrics_improvement: HashMap<String, bool>,
    archive: Archive,
}

impl GenerationKeeper {
    pub fn new(objective: Option<Objective>) -> Self {
        Self::with_keep_n_best(objective, 1)
    }

    pub fn with_keep_n_best(objective: Option<Objective>, keep_n_best: usize) -> Self {
        let archive = if objective
            .as_ref()
            .map(|o| o.is_multi_objective())
            .unwrap_or(false)
        {
            Archive::Pareto(ParetoFront::new(
                keep_n_best * PARETO_MAX_POP_SIZE_MULTIPLIER,
            ))
        } else {
            Archive::Hall(HallOfFame::new(keep_n_best.max(1)))
        };
        let mut keeper = Self {
            objective,
            generation_num: 0,
            stagnation_counter: 0,
            stagnation_start: Instant::now(),
            metrics_improvement: HashMap::new(),
            archive,
        };
        keeper.reset_metrics_improvement();
        keeper
    }

    pub fn with_initial_generation(mut self, population: PopulationT) -> Self {
        self.append(&population);
        self
    }

    pub fn generation_num(&self) -> usize {
        self.generation_num
    }

    pub fn best_individuals(&self) -> Vec<crate::golem::optimisers::history::Individual> {
        self.archive.items()
    }

    pub fn stagnation_iter_count(&self) -> usize {
        self.stagnation_counter
    }

    pub fn stagnation_time_duration(&self) -> f64 {
        self.stagnation_start.elapsed().as_secs_f64() / 60.0
    }

    pub fn is_any_improved(&self) -> bool {
        ImprovementWatcher::is_any_improved(self)
    }

    pub fn is_quality_improved(&self) -> bool {
        ImprovementWatcher::is_quality_improved(self)
    }

    pub fn is_complexity_improved(&self) -> bool {
        ImprovementWatcher::is_complexity_improved(self)
    }

    pub fn append(&mut self, population: &PopulationT) {
        let previous = self.archive_fitness();
        self.archive.update(population);
        self.update_improvements(&previous);
        self.generation_num += 1;
        if self.is_any_improved() || self.generation_num == 1 {
            self.stagnation_start = Instant::now();
            self.stagnation_counter = 0;
        } else {
            self.stagnation_counter += 1;
        }
    }

    fn metric_ids(&self) -> Vec<String> {
        self.objective
            .as_ref()
            .map(|o| {
                let mut ids = o.quality_metrics();
                ids.extend(o.complexity_metrics());
                ids
            })
            .unwrap_or_default()
    }

    fn reset_metrics_improvement(&mut self) {
        self.metrics_improvement = self
            .metric_ids()
            .into_iter()
            .map(|id| (id, false))
            .collect();
    }

    fn archive_fitness(&self) -> HashMap<String, Vec<f64>> {
        let items = self.archive.items();
        if items.is_empty() {
            return HashMap::new();
        }
        let mut result = HashMap::new();
        for (metric_idx, metric_id) in self.metric_ids().iter().enumerate() {
            let values: Vec<f64> = items
                .iter()
                .map(|ind| {
                    ind.fitness
                        .values()
                        .get(metric_idx)
                        .copied()
                        .unwrap_or(f64::INFINITY)
                })
                .collect();
            result.insert(metric_id.clone(), values);
        }
        result
    }

    fn update_improvements(&mut self, previous_metric_archive: &HashMap<String, Vec<f64>>) {
        self.reset_metrics_improvement();
        let current = self.archive_fitness();
        for metric in self.metric_ids() {
            let previous_worst = previous_metric_archive
                .get(&metric)
                .and_then(|v| v.iter().copied().reduce(f64::max))
                .unwrap_or(f64::INFINITY);
            let current_worst = current
                .get(&metric)
                .and_then(|v| v.iter().copied().reduce(f64::max))
                .unwrap_or(f64::INFINITY);
            if is_metric_worse(&[previous_worst], &[current_worst]) {
                self.metrics_improvement.insert(metric.clone(), true);
            }
        }
    }
}

impl ImprovementWatcher for GenerationKeeper {
    fn stagnation_iter_count(&self) -> usize {
        self.stagnation_counter
    }

    fn stagnation_time_duration(&self) -> f64 {
        self.stagnation_start.elapsed().as_secs_f64() / 60.0
    }

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
