use std::sync::{Arc, RwLock};

use crate::golem::dag::{
    distance_to_root_level, Graph, GraphDelegate, LinkedGraphNode,
};
use crate::golem::optimisers::genetic::constants::MAX_GRAPH_GEN_ATTEMPTS;
use crate::golem::optimisers::genetic::params::{GraphRequirements, OptNodeFactory};
use crate::golem::optimisers::genetic::rng::GeneticRng;
use crate::golem::optimisers::genetic::GraphVerifier;

type NodeArc = Arc<RwLock<LinkedGraphNode>>;

#[derive(Clone)]
pub struct RandomGrowthGraphFactory {
    verifier: GraphVerifier,
    node_factory: Arc<OptNodeFactory>,
    rng: GeneticRng,
}

impl RandomGrowthGraphFactory {
    pub fn new(
        verifier: GraphVerifier,
        node_factory: Arc<OptNodeFactory>,
        rng: GeneticRng,
    ) -> Self {
        Self {
            verifier,
            node_factory,
            rng,
        }
    }

    pub fn with_entropy(verifier: GraphVerifier, node_factory: Arc<OptNodeFactory>) -> Self {
        Self::new(verifier, node_factory, GeneticRng::entropy())
    }

    pub fn generate(
        &self,
        requirements: &GraphRequirements,
        max_depth: Option<usize>,
    ) -> GraphDelegate {
        let max_depth = max_depth.unwrap_or(requirements.max_depth);
        for n_iter in 1..=MAX_GRAPH_GEN_ATTEMPTS {
            let graph = self.try_generate(requirements, max_depth);
            if (self.verifier)(&graph) && graph.depth() <= max_depth {
                return graph;
            }
            if n_iter == MAX_GRAPH_GEN_ATTEMPTS {
                return fallback_graph(&self.node_factory);
            }
        }
        fallback_graph(&self.node_factory)
    }

    fn try_generate(&self, requirements: &GraphRequirements, max_depth: usize) -> GraphDelegate {
        let Some(root) = self.node_factory.get_node(false) else {
            return GraphDelegate::new(LinkedGraphNode::from_name("a"));
        };
        let mut graph = GraphDelegate::new(root.clone());
        if requirements.max_depth > 1 {
            graph_growth(
                &mut graph,
                &root,
                &self.node_factory,
                requirements,
                max_depth,
                &self.rng,
            );
        }
        graph
    }
}

fn fallback_graph(node_factory: &OptNodeFactory) -> GraphDelegate {
    if let Some(root) = node_factory.get_node(false) {
        GraphDelegate::new(root)
    } else {
        GraphDelegate::new(LinkedGraphNode::from_name("a"))
    }
}

fn graph_growth(
    graph: &mut GraphDelegate,
    node_parent: &NodeArc,
    node_factory: &OptNodeFactory,
    requirements: &GraphRequirements,
    max_depth: usize,
    rng: &GeneticRng,
) {
    let min_arity = requirements.min_arity.max(1);
    let max_arity = requirements.max_arity.max(min_arity);
    let offspring_size = rng.gen_range(min_arity..=max_arity);

    for _ in 0..offspring_size {
        let Some(node) = node_factory.get_node(false) else {
            continue;
        };
        graph.add_node(node.clone());
        graph.connect_nodes(node_parent, &node);

        let height = distance_to_root_level(graph, &node) as usize;
        let is_max_depth_exceeded = height >= max_depth.saturating_sub(1);
        if !is_max_depth_exceeded && rng.gen_f64() < 0.3 {
            graph_growth(graph, &node, node_factory, requirements, max_depth, rng);
        }
    }
}
