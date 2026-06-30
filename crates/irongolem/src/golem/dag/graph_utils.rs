use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use super::linked_graph::Graph;
use super::linked_graph_node::LinkedGraphNode;

pub fn distance_to_root_level<G: Graph + ?Sized>(
    graph: &G,
    node: &Arc<RwLock<LinkedGraphNode>>,
) -> i32 {
    if graph_has_cycle(graph) {
        return -1;
    }

    let mut height = 0;
    let mut parent_node = node.clone();
    for _ in 0..graph.length() {
        let children = graph.node_children(&parent_node);
        if children.is_empty() {
            return height;
        }
        height += 1;
        parent_node = children[0].clone();
    }
    height
}

pub fn distance_to_primary_level(node: &Arc<RwLock<LinkedGraphNode>>) -> i32 {
    let depth = node_depth(std::slice::from_ref(node));
    if depth > 0 {
        depth - 1
    } else {
        -1
    }
}

pub fn nodes_from_layer<G: Graph + ?Sized>(
    graph: &G,
    layer_number: usize,
) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
    fn get_nodes(
        roots: &[Arc<RwLock<LinkedGraphNode>>],
        current_height: usize,
        layer_number: usize,
    ) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
        if current_height == layer_number {
            roots.to_vec()
        } else {
            let mut nodes = Vec::new();
            for root in roots {
                let parents = root.read().unwrap().nodes_from.clone();
                nodes.extend(get_nodes(&parents, current_height + 1, layer_number));
            }
            nodes
        }
    }

    get_nodes(&graph.root_nodes(), 0, layer_number)
}

pub fn ordered_subnodes_hierarchy(
    node: &Arc<RwLock<LinkedGraphNode>>,
) -> Result<Vec<Arc<RwLock<LinkedGraphNode>>>, String> {
    let mut started: HashSet<usize> = HashSet::from([Arc::as_ptr(node) as usize]);
    let mut visited: HashSet<usize> = HashSet::new();

    fn subtree_impl(
        node: &Arc<RwLock<LinkedGraphNode>>,
        started: &mut HashSet<usize>,
        visited: &mut HashSet<usize>,
    ) -> Result<Vec<Arc<RwLock<LinkedGraphNode>>>, String> {
        let mut nodes = vec![node.clone()];
        let parents = node.read().unwrap().nodes_from.clone();
        for parent in parents {
            let ptr = Arc::as_ptr(&parent) as usize;
            if visited.contains(&ptr) {
                continue;
            } else if started.contains(&ptr) {
                return Err("Can not build ordered node hierarchy: graph has cycle".to_string());
            }
            started.insert(ptr);
            nodes.extend(subtree_impl(&parent, started, visited)?);
            visited.insert(ptr);
        }
        Ok(nodes)
    }

    subtree_impl(node, &mut started, &mut visited)
}

pub fn node_depth(nodes: &[Arc<RwLock<LinkedGraphNode>>]) -> i32 {
    let mut final_depth: HashMap<String, i32> = HashMap::new();
    let mut subnodes: HashSet<String> = HashSet::new();

    for node in nodes {
        let node_uid = node.read().unwrap().uid.clone();
        if subnodes.contains(&node_uid) {
            continue;
        }

        let mut max_depth = 0i32;
        let mut visited_ptrs: Vec<usize> = vec![Arc::as_ptr(node) as usize];
        let mut stack: Vec<(Arc<RwLock<LinkedGraphNode>>, i32, usize)> = vec![(node.clone(), 1, 0)];

        while !stack.is_empty() {
            let (curr_node, depth_now, parent_idx) = {
                let top = stack.last().unwrap();
                (top.0.clone(), top.1, top.2)
            };
            let parents = curr_node.read().unwrap().nodes_from.clone();
            if parent_idx < parents.len() {
                let parent = parents[parent_idx].clone();
                stack.last_mut().unwrap().2 += 1;
                subnodes.insert(parent.read().unwrap().uid.clone());
                let parent_ptr = Arc::as_ptr(&parent) as usize;
                let parent_uid = parent.read().unwrap().uid.clone();
                if let Some(&cached) = final_depth.get(&parent_uid) {
                    max_depth = max_depth.max(depth_now + cached);
                } else if !visited_ptrs.contains(&parent_ptr) {
                    visited_ptrs.push(parent_ptr);
                    stack.push((parent, depth_now + 1, 0));
                } else {
                    return -1;
                }
            } else {
                let (_, depth_now, _) = stack.pop().unwrap();
                visited_ptrs.pop();
                max_depth = max_depth.max(depth_now);
            }
        }

        final_depth.insert(node_uid, max_depth);
    }

    final_depth.values().copied().max().unwrap_or(0)
}

pub fn graph_has_cycle<G: Graph + ?Sized>(graph: &G) -> bool {
    let nodes = graph.nodes();
    let mut visited: HashMap<String, bool> = nodes
        .iter()
        .map(|n| (n.read().unwrap().uid.clone(), false))
        .collect();
    let mut on_stack: HashMap<String, bool> = nodes
        .iter()
        .map(|n| (n.read().unwrap().uid.clone(), false))
        .collect();

    for node in nodes {
        let start_uid = node.read().unwrap().uid.clone();
        if visited[&start_uid] {
            continue;
        }
        let mut stack = vec![node];
        while !stack.is_empty() {
            let cur_uid = stack.last().unwrap().read().unwrap().uid.clone();
            if !visited[&cur_uid] {
                visited.insert(cur_uid.clone(), true);
                on_stack.insert(cur_uid.clone(), true);
            } else {
                on_stack.insert(cur_uid.clone(), false);
                stack.pop();
            }
            if let Some(cur_node) = stack.last() {
                let parents = cur_node.read().unwrap().nodes_from.clone();
                for parent in parents {
                    let parent_uid = parent.read().unwrap().uid.clone();
                    if !visited[&parent_uid] {
                        stack.push(parent);
                    } else if on_stack[&parent_uid] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn get_connected_components<G: Graph + ?Sized>(
    graph: &G,
    nodes: Option<&[Arc<RwLock<LinkedGraphNode>>]>,
) -> Vec<HashSet<usize>> {
    fn bfs<G: Graph + ?Sized>(graph: &G, source: &Arc<RwLock<LinkedGraphNode>>) -> HashSet<usize> {
        let mut seen: HashSet<usize> = HashSet::new();
        let mut nextlevel: HashSet<usize> = HashSet::from([Arc::as_ptr(source) as usize]);

        while !nextlevel.is_empty() {
            let thislevel = nextlevel.clone();
            nextlevel.clear();
            for ptr in thislevel {
                if seen.insert(ptr) {
                    let node = graph
                        .nodes()
                        .into_iter()
                        .find(|n| Arc::as_ptr(n) as usize == ptr);
                    if let Some(v) = node {
                        let parents = v.read().unwrap().nodes_from.clone();
                        for p in parents {
                            nextlevel.insert(Arc::as_ptr(&p) as usize);
                        }
                        for child in graph.node_children(&v) {
                            nextlevel.insert(Arc::as_ptr(&child) as usize);
                        }
                    }
                }
            }
        }

        seen
    }

    let all_nodes = graph.nodes();
    let nodes_to_visit: Vec<Arc<RwLock<LinkedGraphNode>>> = match nodes {
        Some(ns) => ns.to_vec(),
        None => all_nodes,
    };

    let mut visited: HashSet<usize> = HashSet::new();
    let mut components = Vec::new();

    for node in nodes_to_visit {
        let ptr = Arc::as_ptr(&node) as usize;
        if visited.contains(&ptr) {
            continue;
        }
        let component = bfs(graph, &node);
        visited.extend(component.iter().copied());
        components.push(component);
    }

    components
}

pub fn get_nodes_by_name<G: Graph + ?Sized>(
    graph: &G,
    name: &str,
) -> Vec<Arc<RwLock<LinkedGraphNode>>> {
    graph
        .nodes()
        .into_iter()
        .filter(|n| n.read().unwrap().content.name == name)
        .collect()
}

type PathEdge = (Arc<RwLock<LinkedGraphNode>>, Arc<RwLock<LinkedGraphNode>>);

/// Returns all simple paths from `source` to `target`, each as a list of directed edges.
pub fn get_all_simple_paths<G: Graph + ?Sized>(
    graph: &G,
    source: &Arc<RwLock<LinkedGraphNode>>,
    target: &Arc<RwLock<LinkedGraphNode>>,
) -> Vec<Vec<PathEdge>> {
    let mut paths: Vec<Vec<PathEdge>> = Vec::new();
    let mut nodes_children: HashMap<String, Vec<Arc<RwLock<LinkedGraphNode>>>> = HashMap::new();
    nodes_children.insert(
        source.read().unwrap().uid.clone(),
        graph.node_children(source),
    );

    let target_uid = target.read().unwrap().uid.clone();
    let mut visited: Vec<Arc<RwLock<LinkedGraphNode>>> = vec![source.clone()];
    let source_guard = source.read().unwrap();
    let mut neighbors: HashSet<usize> = source_guard
        .nodes_from
        .iter()
        .map(|n| Arc::as_ptr(n) as usize)
        .collect();
    drop(source_guard);
    for child in nodes_children
        .get(&source.read().unwrap().uid)
        .cloned()
        .unwrap_or_default()
    {
        neighbors.insert(Arc::as_ptr(&child) as usize);
    }

    let mut stack: Vec<Vec<usize>> = vec![neighbors.into_iter().collect()];

    while let Some(neighbors) = stack.last_mut() {
        let neighbor_ptr = match neighbors.pop() {
            Some(p) => p,
            None => {
                stack.pop();
                visited.pop();
                continue;
            }
        };

        let neighbor = graph
            .nodes()
            .into_iter()
            .find(|n| Arc::as_ptr(n) as usize == neighbor_ptr);
        let Some(neighbor) = neighbor else {
            continue;
        };

        if visited.iter().any(|n| Arc::ptr_eq(n, &neighbor)) {
            continue;
        }

        if neighbor.read().unwrap().uid == target_uid {
            let path: Vec<_> = visited
                .iter()
                .chain(std::iter::once(&neighbor))
                .cloned()
                .collect();
            let pairs: Vec<PathEdge> = path
                .windows(2)
                .map(|w| (w[0].clone(), w[1].clone()))
                .collect();
            paths.push(pairs);
            continue;
        }

        visited.push(neighbor.clone());
        let uid = neighbor.read().unwrap().uid.clone();
        let children = nodes_children
            .entry(uid.clone())
            .or_insert_with(|| graph.node_children(&neighbor))
            .clone();
        let guard = neighbor.read().unwrap();
        let mut next_neighbors: Vec<usize> = guard
            .nodes_from
            .iter()
            .map(|n| Arc::as_ptr(n) as usize)
            .collect();
        drop(guard);
        for child in children {
            next_neighbors.push(Arc::as_ptr(&child) as usize);
        }
        stack.push(next_neighbors);
    }

    paths
}
