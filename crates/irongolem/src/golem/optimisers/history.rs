use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::fitness::Fitness;
use crate::golem::dag::GraphDelegate;

pub use crate::golem::optimisers::archive::ParetoFront;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentOperator {
    pub type_: String,
    pub operators: String,
    #[serde(skip)]
    pub parent_individuals: Vec<Arc<Individual>>,
    pub parent_uids: Vec<String>,
}

impl ParentOperator {
    pub fn new(
        type_: impl Into<String>,
        operators: impl Into<String>,
        parent_individuals: Vec<Arc<Individual>>,
    ) -> Self {
        let parent_uids = parent_individuals.iter().map(|p| p.uid.clone()).collect();
        Self {
            type_: type_.into(),
            operators: operators.into(),
            parent_individuals,
            parent_uids,
        }
    }

    pub fn parents(&self) -> &[Arc<Individual>] {
        &self.parent_individuals
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Individual {
    pub uid: String,
    #[serde(skip)]
    pub graph: Arc<GraphDelegate>,
    pub fitness: Fitness,
    #[serde(skip)]
    pub parent_operator: Option<ParentOperator>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub native_generation: Option<usize>,
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Eq for Individual {}

impl fmt::Display for Individual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uid)
    }
}

impl Individual {
    pub fn new(graph: Arc<GraphDelegate>) -> Self {
        Self::with_uid(Uuid::new_v4().to_string(), graph)
    }

    pub fn with_uid(uid: impl Into<String>, graph: Arc<GraphDelegate>) -> Self {
        Self {
            uid: uid.into(),
            graph,
            fitness: Fitness::default(),
            parent_operator: None,
            metadata: HashMap::new(),
            native_generation: None,
        }
    }

    pub fn with_fitness(graph: Arc<GraphDelegate>, fitness: Fitness) -> Self {
        Self {
            uid: Uuid::new_v4().to_string(),
            graph,
            fitness,
            parent_operator: None,
            metadata: HashMap::new(),
            native_generation: None,
        }
    }

    pub fn with_parent_operator(mut self, parent_operator: ParentOperator) -> Self {
        self.parent_operator = Some(parent_operator);
        self
    }

    pub fn set_fitness(&mut self, fitness: Fitness) {
        self.fitness = fitness;
    }

    /// Apply an evaluation outcome. Refuses to overwrite a valid fitness
    /// (Python `Individual.set_evaluation_result`).
    pub fn set_evaluation_result(&mut self, eval_result: GraphEvalResult) -> Result<(), String> {
        if self.fitness.is_valid() {
            return Err("The individual has valid fitness and can not be evaluated again.".into());
        }
        self.fitness = eval_result.fitness;
        self.graph = eval_result.graph;
        self.metadata.extend(eval_result.metadata);
        Ok(())
    }

    pub fn set_native_generation(&mut self, native_generation: usize) {
        if self.native_generation.is_none() {
            self.native_generation = Some(native_generation);
        }
    }

    pub fn has_native_generation(&self) -> bool {
        self.native_generation.is_some()
    }

    pub fn parents(&self) -> Vec<Arc<Individual>> {
        self.parent_operator
            .as_ref()
            .map(|op| op.parent_individuals.clone())
            .unwrap_or_default()
    }

    pub fn parents_from_prev_generation(&self) -> Vec<Arc<Individual>> {
        let mut next_parents = self.parents();
        for _ in 0..1_000_000 {
            if next_parents.is_empty() || next_parents.iter().all(|p| p.has_native_generation()) {
                break;
            }
            next_parents = next_parents.into_iter().flat_map(|p| p.parents()).collect();
        }
        next_parents
            .into_iter()
            .filter(|p| p.has_native_generation())
            .collect()
    }

    pub fn operators_from_prev_generation(&self) -> Vec<ParentOperator> {
        let Some(ref op) = self.parent_operator else {
            return Vec::new();
        };
        let parents_from_prev = self.parents_from_prev_generation();
        let mut operators = vec![op.clone()];
        let mut next_parents = self.parents();
        for _ in 0..1_000_000 {
            if next_parents == parents_from_prev {
                break;
            }
            let parents_snapshot = next_parents.clone();
            next_parents = parents_snapshot.iter().flat_map(|p| p.parents()).collect();
            for p in parents_snapshot {
                if let Some(ref parent_op) = p.parent_operator {
                    operators.push(parent_op.clone());
                }
            }
        }
        operators.reverse();
        operators
    }

    pub fn with_graph(mut self, graph: Arc<GraphDelegate>) -> Self {
        self.graph = graph;
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Generation {
    pub individuals: Vec<Individual>,
    pub generation_num: usize,
    pub label: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Generation {
    pub fn new(
        individuals: Vec<Individual>,
        generation_num: usize,
        label: Option<String>,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            individuals,
            generation_num,
            label,
            metadata: metadata.unwrap_or_default(),
        }
    }

    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.individuals.is_empty()
    }
}

/// Outcome of evaluating one graph (Python `GraphEvalResult`).
#[derive(Debug, Clone)]
pub struct GraphEvalResult {
    pub uid_of_individual: String,
    pub fitness: Fitness,
    pub graph: Arc<GraphDelegate>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl GraphEvalResult {
    pub fn new(
        uid_of_individual: impl Into<String>,
        fitness: Fitness,
        graph: Arc<GraphDelegate>,
    ) -> Self {
        Self {
            uid_of_individual: uid_of_individual.into(),
            fitness,
            graph,
            metadata: HashMap::new(),
        }
    }

    /// Python `GraphEvalResult.__bool__` — invalid fitness counts as failed eval.
    pub fn is_successful(&self) -> bool {
        self.fitness.is_valid()
    }
}
