use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use serde_json::Value;

use super::linked_graph::Graph;

#[derive(Debug, Clone, PartialEq)]
pub struct ConvertedNode {
    pub name: String,
    pub params: BTreeMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ConvertedGraph {
    pub nodes: Vec<ConvertedNode>,
    pub edges: Vec<(usize, usize)>,
}

pub fn graph_structure_as_digraph<G: Graph + ?Sized>(graph: &G) -> ConvertedGraph {
    let nodes = graph.nodes();
    let ptr_to_idx: HashMap<usize, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (Arc::as_ptr(n) as usize, i))
        .collect();

    let converted_nodes: Vec<ConvertedNode> = nodes
        .iter()
        .map(|n| {
            let n = n.read().unwrap();
            ConvertedNode {
                name: n.content.name.clone(),
                params: n.content.params.clone(),
            }
        })
        .collect();

    let mut edges = Vec::new();
    for (child_idx, node) in nodes.iter().enumerate() {
        for parent in node.read().unwrap().nodes_from.iter() {
            let parent_idx = ptr_to_idx[&(Arc::as_ptr(parent) as usize)];
            edges.push((parent_idx, child_idx));
        }
    }

    ConvertedGraph {
        nodes: converted_nodes,
        edges,
    }
}

fn nodes_match(a: &ConvertedNode, b: &ConvertedNode) -> bool {
    a.name == b.name && a.params == b.params
}

fn mapping_cost(g1: &ConvertedGraph, g2: &ConvertedGraph, mapping: &[Option<usize>]) -> u32 {
    let mut cost = 0u32;

    for (i, target) in mapping.iter().enumerate() {
        match target {
            None => cost += 1,
            Some(j) => {
                if !nodes_match(&g1.nodes[i], &g2.nodes[*j]) {
                    cost += 1;
                }
            }
        }
    }

    let used: HashSet<usize> = mapping.iter().filter_map(|m| *m).collect();
    cost += (g2.nodes.len().saturating_sub(used.len())) as u32;

    let mut mapped_edges: HashSet<(usize, usize)> = HashSet::new();
    for &(p, c) in &g1.edges {
        match (
            mapping.get(p).copied().flatten(),
            mapping.get(c).copied().flatten(),
        ) {
            (Some(pj), Some(cj)) => {
                mapped_edges.insert((pj, cj));
            }
            _ => cost += 1,
        }
    }

    let g2_edges: HashSet<(usize, usize)> = g2.edges.iter().copied().collect();
    cost += mapped_edges.symmetric_difference(&g2_edges).count() as u32;

    cost
}

fn enumerate_mappings(
    g1: &ConvertedGraph,
    g2: &ConvertedGraph,
    idx: usize,
    mapping: &mut [Option<usize>],
    used: &mut HashSet<usize>,
    best: &mut u32,
) {
    if idx == mapping.len() {
        let cost = mapping_cost(g1, g2, mapping);
        *best = (*best).min(cost);
        return;
    }

    mapping[idx] = None;
    enumerate_mappings(g1, g2, idx + 1, mapping, used, best);

    for j in 0..g2.nodes.len() {
        if used.insert(j) {
            mapping[idx] = Some(j);
            enumerate_mappings(g1, g2, idx + 1, mapping, used, best);
            mapping[idx] = None;
            used.remove(&j);
        }
    }
}

pub fn graph_edit_distance(g1: &ConvertedGraph, g2: &ConvertedGraph) -> u32 {
    if g1.nodes.is_empty() && g2.nodes.is_empty() {
        return 0;
    }

    let mut mapping = vec![None; g1.nodes.len()];
    let mut used = HashSet::new();
    let mut best = u32::MAX;
    enumerate_mappings(g1, g2, 0, &mut mapping, &mut used, &mut best);
    best
}
