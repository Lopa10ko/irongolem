use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::golem::dag::{
    distance_to_primary_level, distance_to_root_level, graph_has_cycle, Graph, GraphDelegate,
    LinkedGraphNode, ReconnectType,
};
use crate::golem::optimisers::advisor::RemoveType;
use crate::golem::optimisers::genetic::params::{
    GPAlgorithmParameters, GraphGenerationParams, OptNodeFactory,
};
use crate::golem::optimisers::genetic::rng::GeneticRng;
use crate::golem::optimisers::genetic::GraphRequirements;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MutationTypesEnum {
    Simple,
    Growth,
    LocalGrowth,
    TreeGrowth,
    Reduce,
    SingleAdd,
    SingleChange,
    SingleDrop,
    SingleEdge,
    None,
}

impl MutationTypesEnum {
    pub fn simple_mutation_set() -> Vec<Self> {
        vec![
            Self::TreeGrowth,
            Self::SingleAdd,
            Self::SingleChange,
            Self::SingleDrop,
            Self::SingleEdge,
        ]
    }

    pub fn rich_mutation_set() -> Vec<Self> {
        vec![Self::Simple, Self::Reduce, Self::Growth, Self::LocalGrowth]
    }
}

pub fn get_mutation_prob(
    mutation_strength: f64,
    node: Option<&Arc<RwLock<LinkedGraphNode>>>,
    default_mutation_prob: f64,
) -> f64 {
    let graph_cycled = match node {
        None => true,
        Some(n) => distance_to_primary_level(n) < 0,
    };
    if graph_cycled {
        return default_mutation_prob;
    }
    let node = node.unwrap();
    mutation_strength / (distance_to_primary_level(node) as f64 + 1.0)
}

pub fn no_mutation(
    graph: GraphDelegate,
    _requirements: &GraphRequirements,
    _graph_gen_params: &GraphGenerationParams,
    _parameters: &GPAlgorithmParameters,
    _rng: &GeneticRng,
) -> GraphDelegate {
    graph
}

pub fn simple_mutation(
    mut graph: GraphDelegate,
    requirements: &GraphRequirements,
    graph_gen_params: &GraphGenerationParams,
    parameters: &GPAlgorithmParameters,
    rng: &GeneticRng,
) -> GraphDelegate {
    let mut visited_nodes: HashSet<usize> = HashSet::new();

    fn replace_node_to_random_recursive(
        graph: &mut GraphDelegate,
        node: &Arc<RwLock<LinkedGraphNode>>,
        node_mutation_probability: f64,
        factory: &OptNodeFactory,
        visited_nodes: &mut HashSet<usize>,
        rng: &GeneticRng,
    ) {
        let node_ptr = Arc::as_ptr(node) as usize;
        if visited_nodes.contains(&node_ptr) {
            return;
        }
        if rng.gen_f64() < node_mutation_probability {
            if let Some(new_node) = factory.exchange_node(node) {
                let parents = node.read().unwrap().nodes_from.clone();
                graph.update_node(node, &new_node);
                visited_nodes.insert(node_ptr);
                visited_nodes.insert(Arc::as_ptr(&new_node) as usize);
                for parent in parents {
                    replace_node_to_random_recursive(
                        graph,
                        &parent,
                        node_mutation_probability,
                        factory,
                        visited_nodes,
                        rng,
                    );
                }
            }
        }
    }

    let root_nodes = graph.root_nodes();
    let root_node = rng.random_choice(&root_nodes);
    let node_mutation_probability =
        get_mutation_prob(parameters.mutation_strength, root_node.as_ref(), 0.7);

    let start_node = if let Some(root) = root_node {
        root
    } else {
        let nodes = graph.nodes();
        rng.random_choice(&nodes)
            .unwrap_or_else(|| LinkedGraphNode::from_name("a"))
    };

    replace_node_to_random_recursive(
        &mut graph,
        &start_node,
        node_mutation_probability,
        &graph_gen_params.node_factory,
        &mut visited_nodes,
        rng,
    );
    let _ = requirements;
    graph
}

pub fn single_edge_mutation(
    mut graph: GraphDelegate,
    requirements: &GraphRequirements,
    _graph_gen_params: &GraphGenerationParams,
    parameters: &GPAlgorithmParameters,
    rng: &GeneticRng,
) -> GraphDelegate {
    fn nodes_not_cycling(
        source_node: &Arc<RwLock<LinkedGraphNode>>,
        target_node: &Arc<RwLock<LinkedGraphNode>>,
    ) -> bool {
        let mut parents = source_node.read().unwrap().nodes_from.clone();
        while !parents.is_empty() {
            if parents.iter().any(|p| Arc::ptr_eq(p, target_node)) {
                return false;
            }
            let mut grandparents = Vec::new();
            for parent in parents {
                grandparents.extend(parent.read().unwrap().nodes_from.clone());
            }
            parents = grandparents;
        }
        true
    }

    if graph.length() < 2 || graph.depth() > requirements.max_depth {
        return graph;
    }

    for _ in 0..parameters.max_num_of_operator_attempts {
        let nodes = graph.nodes();
        let sampled = rng.sample(&nodes, 2);
        if sampled.len() < 2 {
            return graph;
        }
        let source_node = &sampled[0];
        let target_node = &sampled[1];
        let already_connected = target_node
            .read()
            .unwrap()
            .nodes_from
            .iter()
            .any(|p| Arc::ptr_eq(p, source_node));
        if !already_connected
            && (graph_has_cycle(&graph) || nodes_not_cycling(source_node, target_node))
        {
            graph.connect_nodes(source_node, target_node);
            break;
        }
    }
    graph
}

pub fn add_intermediate_node(
    mut graph: GraphDelegate,
    node_factory: &OptNodeFactory,
    rng: &GeneticRng,
) -> GraphDelegate {
    let mut nodes_with_parents: Vec<_> = graph
        .nodes()
        .into_iter()
        .filter(|n| !n.read().unwrap().nodes_from.is_empty())
        .collect();
    if nodes_with_parents.is_empty() {
        return graph;
    }
    rng.shuffle(&mut nodes_with_parents);
    for node_to_mutate in nodes_with_parents {
        let Some(new_node) = node_factory.get_parent_node(&node_to_mutate, false) else {
            continue;
        };
        {
            let parents = node_to_mutate.read().unwrap().nodes_from.clone();
            new_node.write().unwrap().nodes_from = parents;
            node_to_mutate.write().unwrap().nodes_from = vec![new_node.clone()];
        }
        graph.add_node(new_node);
        break;
    }
    graph
}

pub fn add_separate_parent_node(
    mut graph: GraphDelegate,
    node_factory: &OptNodeFactory,
    rng: &GeneticRng,
) -> GraphDelegate {
    let mut indices: Vec<usize> = (0..graph.length()).collect();
    rng.shuffle(&mut indices);
    let nodes = graph.nodes();
    for idx in indices {
        let node_to_mutate = &nodes[idx];
        let Some(new_node) = node_factory.get_parent_node(node_to_mutate, true) else {
            continue;
        };
        {
            let mut guard = node_to_mutate.write().unwrap();
            if guard.nodes_from.is_empty() {
                guard.nodes_from = vec![new_node.clone()];
            } else {
                guard.nodes_from.push(new_node.clone());
            }
        }
        graph.add_node(new_node);
        break;
    }
    graph
}

pub fn add_as_child(
    mut graph: GraphDelegate,
    node_factory: &OptNodeFactory,
    rng: &GeneticRng,
) -> GraphDelegate {
    let mut indices: Vec<usize> = (0..graph.length()).collect();
    rng.shuffle(&mut indices);
    let nodes = graph.nodes();
    for idx in indices {
        let node_to_mutate = &nodes[idx];
        let old_node_children = graph.node_children(node_to_mutate);
        let new_node_child = rng.random_choice(&old_node_children);
        let Some(new_node) = node_factory.get_node(false) else {
            continue;
        };
        graph.add_node(new_node.clone());
        graph.connect_nodes(node_to_mutate, &new_node);
        if let Some(child) = new_node_child {
            graph.connect_nodes(&new_node, &child);
            graph.disconnect_nodes(node_to_mutate, &child, true);
        }
        break;
    }
    graph
}

pub fn single_add_mutation(
    graph: GraphDelegate,
    requirements: &GraphRequirements,
    graph_gen_params: &GraphGenerationParams,
    _parameters: &GPAlgorithmParameters,
    rng: &GeneticRng,
) -> GraphDelegate {
    if graph.depth() >= requirements.max_depth {
        return graph;
    }

    let original = graph.deep_clone();
    let factory = &graph_gen_params.node_factory;
    let mut strategies: Vec<fn(GraphDelegate, &OptNodeFactory, &GeneticRng) -> GraphDelegate> = vec![
        add_as_child,
        add_separate_parent_node,
        add_intermediate_node,
    ];
    rng.shuffle(&mut strategies);

    for strategy in strategies {
        let new_graph = strategy(graph.deep_clone(), factory, rng);
        if new_graph != original {
            return new_graph;
        }
    }
    graph
}

pub fn single_change_mutation(
    mut graph: GraphDelegate,
    _requirements: &GraphRequirements,
    graph_gen_params: &GraphGenerationParams,
    _parameters: &GPAlgorithmParameters,
    rng: &GeneticRng,
) -> GraphDelegate {
    let mut indices: Vec<usize> = (0..graph.length()).collect();
    rng.shuffle(&mut indices);
    let nodes = graph.nodes();
    for idx in indices {
        let node = &nodes[idx];
        if let Some(new_node) = graph_gen_params.node_factory.exchange_node(node) {
            graph.update_node(node, &new_node);
            break;
        }
    }
    graph
}

pub fn single_drop_mutation(
    mut graph: GraphDelegate,
    _requirements: &GraphRequirements,
    graph_gen_params: &GraphGenerationParams,
    parameters: &GPAlgorithmParameters,
    rng: &GeneticRng,
) -> GraphDelegate {
    if graph.length() < 2 {
        return graph;
    }

    for _ in 0..parameters.max_num_of_operator_attempts {
        let nodes = graph.nodes();
        let Some(node_to_del) = rng.random_choice(&nodes) else {
            break;
        };
        match graph_gen_params
            .advisor
            .can_be_removed(&node_to_del, &graph)
        {
            RemoveType::WithParents | RemoveType::WithDirectChildren => {
                graph.delete_subtree(&node_to_del);
                break;
            }
            RemoveType::NodeRewire => {
                graph.delete_node(&node_to_del, ReconnectType::All);
                break;
            }
            RemoveType::NodeOnly => {
                graph.delete_node(&node_to_del, ReconnectType::None);
                break;
            }
            RemoveType::Forbidden => continue,
        }
    }
    graph
}

pub fn reduce_mutation(
    mut graph: GraphDelegate,
    requirements: &GraphRequirements,
    graph_gen_params: &GraphGenerationParams,
    _parameters: &GPAlgorithmParameters,
    rng: &GeneticRng,
) -> GraphDelegate {
    if graph.length() == 1 {
        return graph;
    }

    let root = graph.root_node();
    let mut nodes: Vec<_> = graph
        .nodes()
        .into_iter()
        .filter(|n| root.as_ref().map(|r| !Arc::ptr_eq(r, n)).unwrap_or(true))
        .collect();
    rng.shuffle(&mut nodes);

    for node_to_del in nodes {
        let children = graph.node_children(&node_to_del);
        let is_possible_to_delete = children.iter().all(|child| {
            child.read().unwrap().nodes_from.len().saturating_sub(1) >= requirements.min_arity
        });
        if is_possible_to_delete {
            graph.delete_subtree(&node_to_del);
        } else if let Some(primary_node) = graph_gen_params.node_factory.get_node(true) {
            graph.update_subtree(&node_to_del, &primary_node);
        } else {
            continue;
        }
        break;
    }
    graph
}

pub type MutationFn = fn(
    GraphDelegate,
    &GraphRequirements,
    &GraphGenerationParams,
    &GPAlgorithmParameters,
    &GeneticRng,
) -> GraphDelegate;

pub fn mutation_fn(mutation_type: MutationTypesEnum) -> MutationFn {
    match mutation_type {
        MutationTypesEnum::None => no_mutation,
        MutationTypesEnum::Simple => simple_mutation,
        MutationTypesEnum::SingleAdd => single_add_mutation,
        MutationTypesEnum::SingleChange => single_change_mutation,
        MutationTypesEnum::SingleDrop => single_drop_mutation,
        MutationTypesEnum::SingleEdge => single_edge_mutation,
        MutationTypesEnum::Reduce => reduce_mutation,
        MutationTypesEnum::TreeGrowth => tree_growth,
        MutationTypesEnum::Growth => growth_mutation,
        MutationTypesEnum::LocalGrowth => local_growth_mutation,
    }
}

pub fn tree_growth(
    mut graph: GraphDelegate,
    requirements: &GraphRequirements,
    graph_gen_params: &GraphGenerationParams,
    parameters: &GPAlgorithmParameters,
    rng: &GeneticRng,
) -> GraphDelegate {
    tree_growth_inner(
        &mut graph,
        requirements,
        graph_gen_params,
        parameters,
        true,
        rng,
    );
    graph
}

fn tree_growth_inner(
    graph: &mut GraphDelegate,
    requirements: &GraphRequirements,
    graph_gen_params: &GraphGenerationParams,
    _parameters: &GPAlgorithmParameters,
    local_growth: bool,
    rng: &GeneticRng,
) {
    let mut indices: Vec<usize> = (0..graph.length()).collect();
    rng.shuffle(&mut indices);
    let nodes = graph.nodes();
    let root = graph.root_node();

    if let Some(idx) = indices.into_iter().next() {
        let node_from_graph = &nodes[idx];
        let (max_depth, is_primary_node_selected) = if local_growth {
            let max_depth = distance_to_primary_level(node_from_graph) as usize;
            let is_root = root
                .as_ref()
                .map(|r| Arc::ptr_eq(r, node_from_graph))
                .unwrap_or(false);
            let parents_empty = node_from_graph.read().unwrap().nodes_from.is_empty();
            let is_primary = parents_empty || (!is_root && rng.gen_range(0..2) == 1);
            (max_depth, is_primary)
        } else {
            let max_depth =
                requirements.max_depth - distance_to_root_level(graph, node_from_graph) as usize;
            let is_primary = distance_to_root_level(graph, node_from_graph) as usize
                >= requirements.max_depth
                && rng.gen_range(0..2) == 1;
            (max_depth, is_primary)
        };

        let new_subtree = if is_primary_node_selected {
            graph_gen_params.node_factory.get_node(true)
        } else {
            None
        };

        let new_subtree = match new_subtree {
            Some(n) => n,
            None => graph_gen_params
                .random_graph_factory
                .generate(requirements, Some(max_depth))
                .root_node()
                .unwrap_or_else(|| {
                    graph_gen_params
                        .node_factory
                        .get_node(false)
                        .unwrap_or_else(|| LinkedGraphNode::from_name("a"))
                }),
        };

        graph.update_subtree(node_from_graph, &new_subtree);
    }
}

pub fn growth_mutation(
    graph: GraphDelegate,
    requirements: &GraphRequirements,
    graph_gen_params: &GraphGenerationParams,
    parameters: &GPAlgorithmParameters,
    rng: &GeneticRng,
) -> GraphDelegate {
    if rng.gen_f64() > 0.5 {
        single_add_mutation(graph, requirements, graph_gen_params, parameters, rng)
    } else {
        tree_growth(graph, requirements, graph_gen_params, parameters, rng)
    }
}

pub fn local_growth_mutation(
    graph: GraphDelegate,
    requirements: &GraphRequirements,
    graph_gen_params: &GraphGenerationParams,
    parameters: &GPAlgorithmParameters,
    rng: &GeneticRng,
) -> GraphDelegate {
    let mut graph = graph;
    tree_growth_inner(
        &mut graph,
        requirements,
        graph_gen_params,
        parameters,
        true,
        rng,
    );
    graph
}
