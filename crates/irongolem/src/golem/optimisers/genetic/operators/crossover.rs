use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use super::operator::{OperatorBase, PopulationT};
use crate::golem::dag::{
    get_all_simple_paths, get_connected_components, get_nodes_by_name, node_depth,
    nodes_from_layer, Graph, GraphDelegate, GraphNode, LinkedGraphNode,
};
use crate::golem::optimisers::genetic::gp_operators::{equivalent_subtree, replace_subtrees};
use crate::golem::optimisers::genetic::params::{GPAlgorithmParameters, GraphGenerationParams};
use crate::golem::optimisers::genetic::rng::GeneticRng;
use crate::golem::optimisers::genetic::GraphRequirements;
use crate::golem::optimisers::history::Individual;

type NodeArc = Arc<RwLock<LinkedGraphNode>>;
type NodeEdge = (NodeArc, NodeArc);
type NodeEdgeList = Vec<NodeEdge>;
type NodeList = Vec<NodeArc>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrossoverTypesEnum {
    Subtree,
    OnePoint,
    None,
    Subgraph,
    ExchangeEdges,
    ExchangeParentsOne,
    ExchangeParentsBoth,
}

type CrossoverFn = fn(&mut GraphDelegate, &mut GraphDelegate, usize, &GeneticRng);

#[derive(Debug, Clone)]
pub struct Crossover {
    base: OperatorBase,
    graph_generation_params: GraphGenerationParams,
}

impl Crossover {
    pub fn new(
        parameters: GPAlgorithmParameters,
        requirements: GraphRequirements,
        graph_generation_params: GraphGenerationParams,
    ) -> Self {
        Self {
            base: OperatorBase::new(parameters, requirements),
            graph_generation_params,
        }
    }

    pub fn call(&self, population: PopulationT) -> PopulationT {
        if population.len() == 1 {
            return population;
        }
        let mut new_population = Vec::new();
        for chunk in population.chunks(2) {
            if chunk.len() == 2 {
                let (a, b) = self.crossover_pair(&chunk[0], &chunk[1]);
                new_population.push(a);
                new_population.push(b);
            } else {
                new_population.push(chunk[0].clone());
            }
        }
        new_population
    }

    fn crossover_pair(
        &self,
        ind_first: &Individual,
        ind_second: &Individual,
    ) -> (Individual, Individual) {
        let rng = &self.base.rng;
        let crossover_type = rng
            .random_choice(&self.base.parameters.crossover_types)
            .unwrap_or(CrossoverTypesEnum::OnePoint);

        if !self.will_crossover_be_applied(ind_first, ind_second, crossover_type) {
            return (ind_first.clone(), ind_second.clone());
        }

        let crossover_func = self.crossover_by_type(crossover_type);
        let max_depth = self.base.requirements.max_depth;

        for _ in 0..self.base.parameters.max_num_of_operator_attempts {
            let mut first_object = ind_first.graph.deep_clone();
            let mut second_object = ind_second.graph.deep_clone();
            crossover_func(&mut first_object, &mut second_object, max_depth, rng);

            let are_correct = (self.graph_generation_params.verifier)(&first_object)
                && (self.graph_generation_params.verifier)(&second_object);
            if are_correct {
                return (
                    Individual::new(Arc::new(first_object)),
                    Individual::new(Arc::new(second_object)),
                );
            }
        }

        (ind_first.clone(), ind_second.clone())
    }

    fn will_crossover_be_applied(
        &self,
        ind_first: &Individual,
        ind_second: &Individual,
        crossover_type: CrossoverTypesEnum,
    ) -> bool {
        !ind_first
            .graph
            .as_ref()
            .graphs_equal(ind_second.graph.as_ref())
            && self.base.rng.gen_f64() <= self.base.parameters.crossover_prob
            && crossover_type != CrossoverTypesEnum::None
    }

    fn crossover_by_type(&self, crossover_type: CrossoverTypesEnum) -> CrossoverFn {
        match crossover_type {
            CrossoverTypesEnum::Subtree => subtree_crossover,
            CrossoverTypesEnum::OnePoint => one_point_crossover,
            CrossoverTypesEnum::ExchangeEdges => exchange_edges_crossover,
            CrossoverTypesEnum::ExchangeParentsOne => exchange_parents_one_crossover,
            CrossoverTypesEnum::ExchangeParentsBoth => exchange_parents_both_crossover,
            CrossoverTypesEnum::Subgraph => subgraph_crossover,
            CrossoverTypesEnum::None => |_, _, _, _| {},
        }
    }
}

pub fn subtree_crossover(
    graph_1: &mut GraphDelegate,
    graph_2: &mut GraphDelegate,
    max_depth: usize,
    rng: &GeneticRng,
) {
    let depth_1 = graph_1.depth();
    let depth_2 = graph_2.depth();
    if depth_1 == 0 || depth_2 == 0 {
        return;
    }

    let random_layer_in_graph_first = rng.gen_range(0..depth_1);
    let min_second_layer = if random_layer_in_graph_first == 0 && depth_2 > 1 {
        1
    } else {
        0
    };
    let random_layer_in_graph_second =
        min_second_layer + rng.gen_range(0..depth_2.saturating_sub(min_second_layer).max(1));

    let layer_first_nodes = nodes_from_layer(graph_1, random_layer_in_graph_first);
    let layer_second_nodes = nodes_from_layer(graph_2, random_layer_in_graph_second);
    let Some(node_from_graph_first) = rng.random_choice(&layer_first_nodes) else {
        return;
    };
    let Some(node_from_graph_second) = rng.random_choice(&layer_second_nodes) else {
        return;
    };

    replace_subtrees(
        graph_1,
        graph_2,
        &node_from_graph_first,
        &node_from_graph_second,
        random_layer_in_graph_first,
        random_layer_in_graph_second,
        max_depth,
    );
}

pub fn one_point_crossover(
    graph_first: &mut GraphDelegate,
    graph_second: &mut GraphDelegate,
    max_depth: usize,
    rng: &GeneticRng,
) {
    let pairs_of_nodes = equivalent_subtree(graph_first, graph_second, false);
    if let Some((node_from_graph_first, node_from_graph_second)) =
        rng.random_choice(&pairs_of_nodes)
    {
        let layer_in_graph_first =
            graph_first.depth() - node_depth(std::slice::from_ref(&node_from_graph_first)) as usize;
        let layer_in_graph_second = graph_second.depth()
            - node_depth(std::slice::from_ref(&node_from_graph_second)) as usize;

        replace_subtrees(
            graph_first,
            graph_second,
            &node_from_graph_first,
            &node_from_graph_second,
            layer_in_graph_first,
            layer_in_graph_second,
            max_depth,
        );
    }
}

pub fn exchange_edges_crossover(
    graph_first: &mut GraphDelegate,
    graph_second: &mut GraphDelegate,
    _max_depth: usize,
    rng: &GeneticRng,
) {
    let edges_1 = graph_first.get_edges();
    let edges_2 = graph_second.get_edges();
    if edges_1.is_empty() || edges_2.is_empty() {
        return;
    }

    let count = ((edges_1.len().min(edges_2.len()) as f64) / 2.0).ceil() as usize;
    let choice_edges_1 = rng.sample(&edges_1, count);
    let choice_edges_2 = rng.sample(&edges_2, count);

    for (parent, child) in &choice_edges_1 {
        child
            .write()
            .unwrap()
            .nodes_from
            .retain(|p| !Arc::ptr_eq(p, parent));
    }
    for (parent, child) in &choice_edges_2 {
        child
            .write()
            .unwrap()
            .nodes_from
            .retain(|p| !Arc::ptr_eq(p, parent));
    }

    let old_edges1: HashSet<(usize, usize)> = graph_first
        .get_edges()
        .iter()
        .map(|(p, c)| (Arc::as_ptr(p) as usize, Arc::as_ptr(c) as usize))
        .collect();
    let old_edges2: HashSet<(usize, usize)> = graph_second
        .get_edges()
        .iter()
        .map(|(p, c)| (Arc::as_ptr(p) as usize, Arc::as_ptr(c) as usize))
        .collect();

    let new_edges_2 = find_edges_in_other_graph(&choice_edges_1, graph_second);
    let new_edges_1 = find_edges_in_other_graph(&choice_edges_2, graph_first);

    for (parent, child) in new_edges_1 {
        let key = (Arc::as_ptr(&parent) as usize, Arc::as_ptr(&child) as usize);
        if !old_edges1.contains(&key) {
            graph_first.connect_nodes(&parent, &child);
        }
    }
    for (parent, child) in new_edges_2 {
        let key = (Arc::as_ptr(&parent) as usize, Arc::as_ptr(&child) as usize);
        if !old_edges2.contains(&key) {
            graph_second.connect_nodes(&parent, &child);
        }
    }
}

fn find_edges_in_other_graph(edges: &[NodeEdge], graph: &mut GraphDelegate) -> NodeEdgeList {
    let mut new_edges = Vec::new();
    for (parent, child) in edges {
        let parent_name = parent.read().unwrap().content.name.clone();
        let child_name = child.read().unwrap().content.name.clone();
        let parent_new = get_or_create_node(graph, &parent_name);
        let child_new = get_or_create_node(graph, &child_name);
        new_edges.push((parent_new, child_new));
    }
    new_edges
}

fn get_or_create_node(graph: &mut GraphDelegate, name: &str) -> Arc<RwLock<LinkedGraphNode>> {
    if let Some(node) = get_nodes_by_name(graph, name).into_iter().next() {
        node
    } else {
        let node = LinkedGraphNode::from_name(name);
        graph.add_node(node.clone());
        node
    }
}

pub fn exchange_parents_one_crossover(
    graph_first: &mut GraphDelegate,
    graph_second: &mut GraphDelegate,
    _max_depth: usize,
    rng: &GeneticRng,
) {
    let nodes_with_parent_or_child = sorted_nodes_from_edges(graph_second);
    if nodes_with_parent_or_child.is_empty() {
        return;
    }

    let selected_node = rng.random_choice(&nodes_with_parent_or_child).unwrap();
    let parents = selected_node.read().unwrap().nodes_from.clone();
    let node_from_first_graph =
        find_nodes_in_other_graph(std::slice::from_ref(&selected_node), graph_first)[0].clone();

    for parent in &parents {
        graph_first.disconnect_nodes(parent, &node_from_first_graph, false);
    }
    let old_edges1: HashSet<(usize, usize)> = graph_first
        .get_edges()
        .iter()
        .map(|(p, c)| (Arc::as_ptr(p) as usize, Arc::as_ptr(c) as usize))
        .collect();

    if !parents.is_empty() {
        let parents_in_first_graph = find_nodes_in_other_graph(&parents, graph_first);
        for parent in parents_in_first_graph {
            let key = (
                Arc::as_ptr(&parent) as usize,
                Arc::as_ptr(&node_from_first_graph) as usize,
            );
            if !old_edges1.contains(&key) {
                graph_first.connect_nodes(&parent, &node_from_first_graph);
            }
        }
    }
}

pub fn exchange_parents_both_crossover(
    graph_first: &mut GraphDelegate,
    graph_second: &mut GraphDelegate,
    _max_depth: usize,
    rng: &GeneticRng,
) {
    let nodes_with_parent_or_child = sorted_nodes_from_edges(graph_second);
    if nodes_with_parent_or_child.is_empty() {
        return;
    }

    let selected_node2 = rng.random_choice(&nodes_with_parent_or_child).unwrap();
    let parents2 = selected_node2.read().unwrap().nodes_from.clone();

    let parents_in_first_graph = if parents2.is_empty() {
        Vec::new()
    } else {
        find_nodes_in_other_graph(&parents2, graph_first)
    };

    let selected_node1 =
        find_nodes_in_other_graph(std::slice::from_ref(&selected_node2), graph_first)[0].clone();
    let parents1 = selected_node1.read().unwrap().nodes_from.clone();

    let parents_in_second_graph = if parents1.is_empty() {
        Vec::new()
    } else {
        find_nodes_in_other_graph(&parents1, graph_second)
    };

    for parent in &parents1 {
        graph_first.disconnect_nodes(parent, &selected_node1, false);
    }
    for parent in &parents2 {
        graph_second.disconnect_nodes(parent, &selected_node2, false);
    }

    let old_edges1: HashSet<(usize, usize)> = graph_first
        .get_edges()
        .iter()
        .map(|(p, c)| (Arc::as_ptr(p) as usize, Arc::as_ptr(c) as usize))
        .collect();
    let old_edges2: HashSet<(usize, usize)> = graph_second
        .get_edges()
        .iter()
        .map(|(p, c)| (Arc::as_ptr(p) as usize, Arc::as_ptr(c) as usize))
        .collect();

    for parent in parents_in_first_graph {
        let key = (
            Arc::as_ptr(&parent) as usize,
            Arc::as_ptr(&selected_node1) as usize,
        );
        if !old_edges1.contains(&key) {
            graph_first.connect_nodes(&parent, &selected_node1);
        }
    }

    for parent in parents_in_second_graph {
        let key = (
            Arc::as_ptr(&parent) as usize,
            Arc::as_ptr(&selected_node2) as usize,
        );
        if !old_edges2.contains(&key) {
            graph_second.connect_nodes(&parent, &selected_node2);
        }
    }
}

fn find_nodes_in_other_graph(
    nodes: &[Arc<RwLock<LinkedGraphNode>>],
    graph: &mut GraphDelegate,
) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
    nodes
        .iter()
        .map(|node| {
            let name = node.read().unwrap().content.name.clone();
            get_or_create_node(graph, &name)
        })
        .collect()
}

fn sorted_nodes_from_edges(graph: &GraphDelegate) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
    let mut node_ptrs: HashSet<usize> = HashSet::new();
    for (parent, child) in graph.get_edges() {
        node_ptrs.insert(Arc::as_ptr(&parent) as usize);
        node_ptrs.insert(Arc::as_ptr(&child) as usize);
    }
    let mut result: Vec<_> = graph
        .nodes()
        .into_iter()
        .filter(|n| node_ptrs.contains(&(Arc::as_ptr(n) as usize)))
        .collect();
    result.sort_by(|a, b| {
        a.read()
            .unwrap()
            .descriptive_id()
            .cmp(&b.read().unwrap().descriptive_id())
    });
    result
}

pub fn subgraph_crossover(
    graph_first: &mut GraphDelegate,
    graph_second: &mut GraphDelegate,
    _max_depth: usize,
    rng: &GeneticRng,
) {
    let (first_subgraphs, first_div_points) = get_subgraphs(graph_first, rng);
    let (second_subgraphs, second_div_points) = get_subgraphs(graph_second, rng);
    *graph_first = connect_subgraphs(
        &first_subgraphs[0],
        &second_subgraphs[1],
        &first_div_points,
        &second_div_points,
        rng,
    );
    *graph_second = connect_subgraphs(
        &first_subgraphs[1],
        &second_subgraphs[0],
        &first_div_points,
        &second_div_points,
        rng,
    );
}

fn get_subgraphs(graph: &mut GraphDelegate, rng: &GeneticRng) -> (Vec<NodeList>, HashSet<usize>) {
    let edges = graph.get_edges();
    if edges.is_empty() {
        let nodes = graph.nodes();
        let div: HashSet<usize> = nodes.iter().map(|n| Arc::as_ptr(n) as usize).collect();
        return (vec![nodes.clone(), nodes], div);
    }

    let (target, source) = rng.random_choice(&edges).unwrap();
    graph.disconnect_nodes(&target, &source, false);

    let mut simple_paths = get_all_simple_paths(graph, &source, &target);
    simple_paths.sort_by_key(|path| path.len());
    let mut division_points: HashSet<usize> =
        HashSet::from([Arc::as_ptr(&source) as usize, Arc::as_ptr(&target) as usize]);

    while !simple_paths.is_empty() {
        let Some(path) = rng.random_choice(&simple_paths) else {
            break;
        };
        if let Some((node_first, node_second)) = path.first().cloned() {
            let node_first_in_second = node_second
                .read()
                .unwrap()
                .nodes_from
                .iter()
                .any(|p| Arc::ptr_eq(p, &node_first));
            if node_first_in_second {
                graph.disconnect_nodes(&node_first, &node_second, false);
            } else {
                graph.disconnect_nodes(&node_second, &node_first, false);
            }
            division_points.insert(Arc::as_ptr(&node_first) as usize);
            division_points.insert(Arc::as_ptr(&node_second) as usize);
        }
        simple_paths = get_all_simple_paths(graph, &source, &target);
        simple_paths.sort_by_key(|path| path.len());
    }

    let components = get_connected_components(graph, Some(&[source.clone(), target.clone()]));
    let ptr_to_node: HashMap<usize, Arc<RwLock<LinkedGraphNode>>> = graph
        .nodes()
        .into_iter()
        .map(|n| (Arc::as_ptr(&n) as usize, n))
        .collect();

    let subgraphs: Vec<Vec<Arc<RwLock<LinkedGraphNode>>>> = components
        .into_iter()
        .map(|component| {
            component
                .into_iter()
                .filter_map(|ptr| ptr_to_node.get(&ptr).cloned())
                .collect()
        })
        .collect();

    let subgraphs = if subgraphs.len() >= 2 {
        subgraphs
    } else if subgraphs.len() == 1 {
        vec![subgraphs[0].clone(), subgraphs[0].clone()]
    } else {
        let nodes = graph.nodes();
        vec![nodes.clone(), nodes]
    };

    (subgraphs, division_points)
}

fn connect_subgraphs(
    first_subgraph: &[Arc<RwLock<LinkedGraphNode>>],
    second_subgraph: &[Arc<RwLock<LinkedGraphNode>>],
    first_div_points: &HashSet<usize>,
    second_div_points: &HashSet<usize>,
    rng: &GeneticRng,
) -> GraphDelegate {
    let mut first_points: Vec<_> = first_subgraph
        .iter()
        .filter(|n| first_div_points.contains(&(Arc::as_ptr(n) as usize)))
        .cloned()
        .collect();
    let mut second_points: Vec<_> = second_subgraph
        .iter()
        .filter(|n| second_div_points.contains(&(Arc::as_ptr(n) as usize)))
        .cloned()
        .collect();

    let connections_num = first_points.len().min(second_points.len());
    let mut new_graph = GraphDelegate::empty();
    for node in first_subgraph.iter().chain(second_subgraph.iter()) {
        new_graph.add_node(node.clone());
    }

    for _ in 0..connections_num {
        let first_idx = rng.gen_range(0..first_points.len());
        let second_idx = rng.gen_range(0..second_points.len());
        let first_node = first_points.remove(first_idx);
        let second_node = second_points.remove(second_idx);

        if rng.gen_f64() > 0.5 {
            new_graph.connect_nodes(&first_node, &second_node);
        } else {
            new_graph.connect_nodes(&second_node, &first_node);
        }
    }

    new_graph
}
