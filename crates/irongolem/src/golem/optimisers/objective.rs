use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::fitness::{to_fitness, Fitness};
use crate::golem::dag::GraphDelegate;

pub type MetricFn = Arc<dyn Fn(Arc<GraphDelegate>) -> f64 + Send + Sync>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectiveInfo {
    #[serde(default)]
    pub is_multi_objective: bool,
    #[serde(default)]
    pub metric_names: Vec<String>,
}

pub struct Objective {
    quality_metrics: HashMap<String, String>,
    complexity_metrics: HashMap<String, String>,
    is_multi_objective: bool,
    evaluators: HashMap<String, MetricFn>,
}

impl Objective {
    pub fn new(metrics: HashMap<String, String>) -> Self {
        Self {
            quality_metrics: metrics,
            complexity_metrics: HashMap::new(),
            is_multi_objective: false,
            evaluators: HashMap::new(),
        }
    }

    pub fn multi_objective(
        quality_metrics: HashMap<String, String>,
        complexity_metrics: HashMap<String, String>,
    ) -> Self {
        Self {
            quality_metrics,
            complexity_metrics,
            is_multi_objective: true,
            evaluators: HashMap::new(),
        }
    }

    pub fn with_evaluator(mut self, name: impl Into<String>, evaluator: MetricFn) -> Self {
        self.evaluators.insert(name.into(), evaluator);
        self
    }

    pub fn metric_names(&self) -> Vec<String> {
        let mut names: Vec<_> = self
            .quality_metrics
            .keys()
            .chain(self.complexity_metrics.keys())
            .cloned()
            .collect();
        names.sort();
        names.dedup();
        names
    }

    pub fn quality_metrics(&self) -> Vec<String> {
        self.quality_metrics.keys().cloned().collect()
    }

    pub fn complexity_metrics(&self) -> Vec<String> {
        self.complexity_metrics.keys().cloned().collect()
    }

    pub fn is_multi_objective(&self) -> bool {
        self.is_multi_objective
    }

    pub fn get_info(&self) -> ObjectiveInfo {
        ObjectiveInfo {
            is_multi_objective: self.is_multi_objective,
            metric_names: self.metric_names(),
        }
    }

    pub fn evaluate(&self, graph: Arc<GraphDelegate>) -> Fitness {
        let values: Vec<f64> = self
            .metric_names()
            .iter()
            .map(|name| {
                self.evaluators
                    .get(name)
                    .map(|f| f(graph.clone()))
                    .unwrap_or(0.0)
            })
            .collect();
        if values.is_empty() {
            return Fitness::valid_fitness();
        }
        to_fitness(&values, self.is_multi_objective)
    }
}

impl Clone for Objective {
    fn clone(&self) -> Self {
        Self {
            quality_metrics: self.quality_metrics.clone(),
            complexity_metrics: self.complexity_metrics.clone(),
            is_multi_objective: self.is_multi_objective,
            evaluators: self.evaluators.clone(),
        }
    }
}

pub struct ObjectiveEvaluate {
    pub objective: Objective,
}

impl ObjectiveEvaluate {
    pub fn new(objective: Objective) -> Self {
        Self { objective }
    }

    pub fn evaluate(&self, graph: Arc<GraphDelegate>) -> Fitness {
        self.objective.evaluate(graph)
    }
}

impl std::ops::Deref for ObjectiveEvaluate {
    type Target = Objective;

    fn deref(&self) -> &Self::Target {
        &self.objective
    }
}
