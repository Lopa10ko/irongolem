use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{Arc, RwLock};

use super::operators::base_mutations::MutationTypesEnum;
use super::operators::crossover::CrossoverTypesEnum;
use super::operators::elitism::ElitismTypesEnum;
use crate::golem::dag::{
    has_no_cycle, has_no_isolated_components, has_no_isolated_nodes, has_no_self_cycled_nodes,
    GraphDelegate, LinkedGraphNode,
};
use crate::golem::optimisers::advisor::{DefaultAdvisor, GraphAdvisor};
use crate::golem::optimisers::genetic::rng::GeneticRng;
use crate::golem::optimisers::random_graph_factory::RandomGrowthGraphFactory;
use serde::{Deserialize, Serialize};

type NodeArc = Arc<RwLock<LinkedGraphNode>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphRequirements {
    pub max_depth: usize,
    pub min_arity: usize,
    pub max_arity: usize,
    pub static_individual_metadata: Option<serde_json::Value>,
}

impl Default for GraphRequirements {
    fn default() -> Self {
        Self {
            max_depth: 5,
            min_arity: 1,
            max_arity: 4,
            static_individual_metadata: None,
        }
    }
}

pub type GraphVerifier = Arc<dyn Fn(&GraphDelegate) -> bool + Send + Sync>;

#[derive(Clone)]
pub struct GraphGenerationParams {
    pub available_node_types: Vec<String>,
    pub node_factory: Arc<OptNodeFactory>,
    pub verifier: GraphVerifier,
    pub advisor: Arc<dyn GraphAdvisor>,
    pub random_graph_factory: Arc<RandomGrowthGraphFactory>,
}

impl std::fmt::Debug for GraphGenerationParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphGenerationParams")
            .field("available_node_types", &self.available_node_types)
            .finish_non_exhaustive()
    }
}

impl GraphGenerationParams {
    pub fn new(available_node_types: Vec<String>) -> Self {
        let factory = Arc::new(OptNodeFactory::new(available_node_types.clone()));
        let verifier = default_graph_verifier();
        let random_graph_factory = Arc::new(RandomGrowthGraphFactory::with_entropy(
            verifier.clone(),
            factory.clone(),
        ));
        Self {
            available_node_types,
            node_factory: factory,
            verifier,
            advisor: Arc::new(DefaultAdvisor),
            random_graph_factory,
        }
    }

    pub fn with_rng(mut self, rng: GeneticRng) -> Self {
        self.random_graph_factory = Arc::new(RandomGrowthGraphFactory::new(
            self.verifier.clone(),
            self.node_factory.clone(),
            rng,
        ));
        self
    }
}

pub fn default_graph_verifier() -> GraphVerifier {
    Arc::new(|graph: &GraphDelegate| {
        has_no_cycle(graph).is_ok()
            && has_no_isolated_nodes(graph).is_ok()
            && has_no_self_cycled_nodes(graph).is_ok()
            && has_no_isolated_components(graph).is_ok()
    })
}

#[derive(Debug)]
pub struct OptNodeFactory {
    available: Vec<String>,
    counter: AtomicUsize,
}

impl Clone for OptNodeFactory {
    fn clone(&self) -> Self {
        Self {
            available: self.available.clone(),
            counter: AtomicUsize::new(self.counter.load(AtomicOrdering::Relaxed)),
        }
    }
}

impl OptNodeFactory {
    pub fn new(available: Vec<String>) -> Self {
        Self {
            available,
            counter: AtomicUsize::new(0),
        }
    }

    pub fn get_all_available_operations(&self) -> Vec<String> {
        self.available.clone()
    }

    pub fn get_node(&self, is_primary: bool) -> Option<NodeArc> {
        let _ = is_primary;
        self.exchange_name(None)
    }

    pub fn get_parent_node(&self, _child: &NodeArc, is_primary: bool) -> Option<NodeArc> {
        self.get_node(is_primary)
    }

    pub fn exchange_node(&self, node: &NodeArc) -> Option<NodeArc> {
        let name = node.read().unwrap().content.name.clone();
        self.exchange_name(Some(&name))
    }

    fn exchange_name(&self, current: Option<&str>) -> Option<NodeArc> {
        let alternatives: Vec<_> = self
            .available
            .iter()
            .filter(|n| current.map(|c| c != n.as_str()).unwrap_or(true))
            .collect();
        if alternatives.is_empty() {
            return None;
        }
        let idx = self.counter.fetch_add(1, AtomicOrdering::Relaxed) % alternatives.len();
        Some(crate::golem::dag::LinkedGraphNode::from_name(
            alternatives[idx],
        ))
    }
}

pub type CustomSelectionFn = Arc<
    dyn Fn(
            &[crate::golem::optimisers::history::Individual],
            usize,
        ) -> Vec<crate::golem::optimisers::history::Individual>
        + Send
        + Sync,
>;

#[derive(Clone)]
pub enum SelectionType {
    Tournament,
    Spea2,
    Custom(CustomSelectionFn),
}

#[derive(Clone)]
pub enum CrossoverType {
    Enum(CrossoverTypesEnum),
}

#[derive(Clone)]
pub enum MutationType {
    Enum(MutationTypesEnum),
}

#[derive(Clone)]
pub struct GPAlgorithmParameters {
    pub pop_size: usize,
    pub multi_objective: bool,
    pub crossover_prob: f64,
    pub mutation_prob: f64,
    pub variable_mutation_num: bool,
    pub max_num_of_operator_attempts: usize,
    pub min_pop_size_with_elitism: usize,
    pub required_valid_ratio: f64,
    pub offspring_rate: f64,
    pub max_pop_size: Option<usize>,
    pub selection_types: Vec<SelectionType>,
    pub crossover_types: Vec<CrossoverTypesEnum>,
    pub mutation_types: Vec<MutationTypesEnum>,
    pub elitism_type: ElitismTypesEnum,
    pub mutation_strength: f64,
    pub random_seed: Option<u64>,
}

impl std::fmt::Debug for GPAlgorithmParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GPAlgorithmParameters")
            .field("pop_size", &self.pop_size)
            .field("multi_objective", &self.multi_objective)
            .field("crossover_prob", &self.crossover_prob)
            .field("mutation_prob", &self.mutation_prob)
            .field("required_valid_ratio", &self.required_valid_ratio)
            .field("offspring_rate", &self.offspring_rate)
            .field("max_pop_size", &self.max_pop_size)
            .finish_non_exhaustive()
    }
}

impl Default for GPAlgorithmParameters {
    fn default() -> Self {
        Self {
            pop_size: 20,
            multi_objective: false,
            crossover_prob: 0.8,
            mutation_prob: 0.8,
            variable_mutation_num: true,
            max_num_of_operator_attempts: 100,
            min_pop_size_with_elitism: 5,
            required_valid_ratio: 0.9,
            offspring_rate: 0.2,
            max_pop_size: Some(100),
            selection_types: vec![SelectionType::Tournament],
            crossover_types: vec![CrossoverTypesEnum::OnePoint],
            mutation_types: MutationTypesEnum::simple_mutation_set(),
            elitism_type: ElitismTypesEnum::KeepNBest,
            mutation_strength: 1.0,
            random_seed: None,
        }
    }
}

impl GPAlgorithmParameters {
    pub fn new(pop_size: usize) -> Self {
        Self {
            pop_size,
            ..Default::default()
        }
    }

    pub fn with_selection_types(mut self, types: Vec<SelectionType>) -> Self {
        self.selection_types = types;
        self
    }

    pub fn with_crossover_types(mut self, types: Vec<CrossoverTypesEnum>) -> Self {
        self.crossover_types = types;
        self
    }

    pub fn with_mutation_types(mut self, types: Vec<MutationTypesEnum>) -> Self {
        self.mutation_types = types;
        self
    }

    pub fn with_elitism_type(mut self, elitism_type: ElitismTypesEnum) -> Self {
        self.elitism_type = elitism_type;
        self
    }

    pub fn with_multi_objective(mut self, multi_objective: bool) -> Self {
        self.multi_objective = multi_objective;
        if multi_objective {
            self.selection_types = vec![SelectionType::Spea2];
        }
        self
    }

    pub fn with_crossover_prob(mut self, prob: f64) -> Self {
        self.crossover_prob = prob;
        self
    }

    pub fn with_mutation_prob(mut self, prob: f64) -> Self {
        self.mutation_prob = prob;
        self
    }

    pub fn with_random_seed(mut self, seed: u64) -> Self {
        self.random_seed = Some(seed);
        self
    }
}
