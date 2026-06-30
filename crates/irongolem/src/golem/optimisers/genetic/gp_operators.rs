use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use crate::golem::dag::{
    clone_subtree, descriptive_id_recursive_nodes, distance_to_primary_level, Graph, GraphDelegate,
    GraphNode, LinkedGraphNode,
};
use crate::golem::optimisers::history::{Individual, ParetoFront};

pub type NodePair = (Arc<RwLock<LinkedGraphNode>>, Arc<RwLock<LinkedGraphNode>>);

pub fn node_is_primary<G: Graph>(graph: &G, node: &Arc<RwLock<LinkedGraphNode>>) -> bool {
    let guard = node.read().unwrap();
    if let Some(serde_json::Value::Bool(is_primary)) = guard.content.params.get("is_primary") {
        return *is_primary;
    }
    let is_root = graph
        .root_node()
        .map(|r| Arc::ptr_eq(&r, node))
        .unwrap_or(false);
    is_root || !guard.nodes_from.is_empty()
}

fn node_ptr(node: &Arc<RwLock<LinkedGraphNode>>) -> usize {
    Arc::as_ptr(node) as usize
}

pub fn equivalent_subtree(
    graph_first: &GraphDelegate,
    graph_second: &GraphDelegate,
    with_primary_nodes: bool,
) -> Vec<NodePair> {
    let nodes_first = graph_first.nodes();
    let nodes_second = graph_second.nodes();
    let mut all_recursive_ids: HashMap<usize, HashSet<usize>> = HashMap::new();
    for node in nodes_first.iter().chain(nodes_second.iter()) {
        let subtree = descriptive_id_recursive_nodes(node, &mut Vec::new());
        all_recursive_ids.insert(node_ptr(node), subtree.iter().map(node_ptr).collect());
    }

    let mut pairs_list = Vec::new();
    let mut pair_set: HashSet<(usize, usize)> = HashSet::new();

    for node_first in &nodes_first {
        for node_second in &nodes_second {
            let key = (node_ptr(node_first), node_ptr(node_second));
            if pair_set.contains(&key) {
                continue;
            }
            let equivalent_pairs = structural_equivalent_nodes(
                graph_first,
                graph_second,
                node_first,
                node_second,
                Some(&all_recursive_ids),
                &mut Vec::new(),
            );
            for pair in equivalent_pairs {
                pair_set.insert((node_ptr(&pair.0), node_ptr(&pair.1)));
                pairs_list.push(pair);
            }
        }
    }

    pairs_list.sort_by(|a, b| {
        let id_a0 = a.0.read().unwrap().descriptive_id();
        let id_a1 = a.1.read().unwrap().descriptive_id();
        let id_b0 = b.0.read().unwrap().descriptive_id();
        let id_b1 = b.1.read().unwrap().descriptive_id();
        (id_a0, id_a1).cmp(&(id_b0, id_b1))
    });

    let mut deduped = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for pair in pairs_list {
        let key = (node_ptr(&pair.0), node_ptr(&pair.1));
        if seen.insert(key) {
            deduped.push(pair);
        }
    }

    if with_primary_nodes {
        return deduped;
    }

    deduped
        .into_iter()
        .filter(|pair| !pair.0.read().unwrap().nodes_from.is_empty())
        .collect()
}

pub fn replace_subtrees(
    graph_first: &mut GraphDelegate,
    graph_second: &mut GraphDelegate,
    node_from_first: &Arc<RwLock<LinkedGraphNode>>,
    node_from_second: &Arc<RwLock<LinkedGraphNode>>,
    layer_in_first: usize,
    layer_in_second: usize,
    max_depth: usize,
) {
    let node_from_graph_first_copy = clone_subtree(node_from_first);

    let summary_depth = layer_in_first + distance_to_primary_level(node_from_second) as usize + 1;
    if summary_depth <= max_depth && summary_depth != 0 {
        graph_first.update_subtree(node_from_first, node_from_second);
    }

    let summary_depth = layer_in_second + distance_to_primary_level(node_from_first) as usize + 1;
    if summary_depth <= max_depth && summary_depth != 0 {
        graph_second.update_subtree(node_from_second, &node_from_graph_first_copy);
    }
}

pub fn num_of_parents_in_crossover(num_of_final_inds: usize) -> usize {
    if num_of_final_inds % 2 == 0 {
        num_of_final_inds
    } else {
        num_of_final_inds + 1
    }
}

pub fn filter_duplicates(archive: &ParetoFront, population: &[Individual]) -> Vec<Individual> {
    archive
        .items
        .iter()
        .filter(|ind| {
            !population
                .iter()
                .any(|pop_ind| ind.fitness == pop_ind.fitness)
        })
        .cloned()
        .collect()
}

pub fn structural_equivalent_nodes(
    graph_first: &GraphDelegate,
    graph_second: &GraphDelegate,
    node_first: &Arc<RwLock<LinkedGraphNode>>,
    node_second: &Arc<RwLock<LinkedGraphNode>>,
    recursive_ids: Option<&HashMap<usize, HashSet<usize>>>,
    seen: &mut Vec<usize>,
) -> Vec<NodePair> {
    let mut nodes = Vec::new();
    let first_ptr = node_ptr(node_first);
    let second_ptr = node_ptr(node_second);

    if seen.contains(&first_ptr) || seen.contains(&second_ptr) {
        return nodes;
    }
    seen.push(first_ptr);
    seen.push(second_ptr);

    let parents_first = node_first.read().unwrap().nodes_from.clone();
    let parents_second = node_second.read().unwrap().nodes_from.clone();

    for node1_child in &parents_first {
        for node2_child in &parents_second {
            let nodes_set = structural_equivalent_nodes(
                graph_first,
                graph_second,
                node1_child,
                node2_child,
                recursive_ids,
                seen,
            );
            nodes.extend(nodes_set);
        }
    }

    if parents_first.len() == parents_second.len()
        && are_subtrees_the_same(&nodes, node_first, node_second, recursive_ids)
        && node_is_primary(graph_first, node_first) == node_is_primary(graph_second, node_second)
    {
        nodes.push((node_first.clone(), node_second.clone()));
    }

    nodes
}

pub fn are_subtrees_the_same(
    match_set: &[NodePair],
    node_first: &Arc<RwLock<LinkedGraphNode>>,
    node_second: &Arc<RwLock<LinkedGraphNode>>,
    recursive_ids: Option<&HashMap<usize, HashSet<usize>>>,
) -> bool {
    let parents_first = node_first.read().unwrap().nodes_from.clone();
    let parents_second = node_second.read().unwrap().nodes_from.clone();

    let (first_recursive_id, second_recursive_id) = match recursive_ids {
        Some(ids) => (
            ids.get(&node_ptr(node_first)).cloned().unwrap_or_default(),
            ids.get(&node_ptr(node_second)).cloned().unwrap_or_default(),
        ),
        None => {
            let first = descriptive_id_recursive_nodes(node_first, &mut Vec::new());
            let second = descriptive_id_recursive_nodes(node_second, &mut Vec::new());
            (
                first.iter().map(node_ptr).collect(),
                second.iter().map(node_ptr).collect(),
            )
        }
    };

    if parents_first.len() != parents_second.len()
        || (match_set.is_empty() && !parents_first.is_empty())
        || first_recursive_id.len() != second_recursive_id.len()
    {
        return false;
    }

    let match_ptrs: HashSet<(usize, usize)> = match_set
        .iter()
        .flat_map(|(a, b)| [(node_ptr(a), node_ptr(b)), (node_ptr(b), node_ptr(a))])
        .collect();

    let mut matched = 0usize;
    for node in &parents_first {
        for node2 in &parents_second {
            if match_ptrs.contains(&(node_ptr(node), node_ptr(node2))) {
                matched += 1;
            }
        }
    }

    matched >= parents_first.len()
}
