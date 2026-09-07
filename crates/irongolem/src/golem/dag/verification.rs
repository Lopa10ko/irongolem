use std::collections::HashSet;
use std::sync::Arc;

use super::graph_delegate::GraphDelegate;
use super::graph_utils::{get_connected_components, graph_has_cycle};
use super::linked_graph::Graph;

/// Native DAG verification rules registered with the adapter (Python `DEFAULT_DAG_RULES`
/// minus `has_root` / `has_one_root`, which are not used by the current Rust verifier).
pub type DagRule = fn(&GraphDelegate) -> Result<(), String>;

pub fn default_dag_rules() -> [DagRule; 4] {
    [
        has_no_cycle as DagRule,
        has_no_isolated_nodes as DagRule,
        has_no_self_cycled_nodes as DagRule,
        has_no_isolated_components as DagRule,
    ]
}

pub const ERROR_PREFIX: &str = "Invalid graph configuration:";

pub fn has_no_cycle<G: Graph + ?Sized>(graph: &G) -> Result<(), String> {
    if graph_has_cycle(graph) {
        return Err(format!("{ERROR_PREFIX} Graph has cycles"));
    }
    Ok(())
}

pub fn has_no_isolated_nodes<G: Graph + ?Sized>(graph: &G) -> Result<(), String> {
    if graph.length() == 1 {
        return Ok(());
    }

    let nodes = graph.nodes();
    let node_ptrs: HashSet<usize> = nodes.iter().map(|n| Arc::as_ptr(n) as usize).collect();

    let mut incident: HashSet<usize> = HashSet::new();
    for edge in graph.get_edges() {
        incident.insert(Arc::as_ptr(&edge.0) as usize);
        incident.insert(Arc::as_ptr(&edge.1) as usize);
    }

    let isolated_count = node_ptrs.difference(&incident).count();
    if isolated_count > 0 {
        return Err(format!("{ERROR_PREFIX} Graph has isolated nodes"));
    }
    Ok(())
}

pub fn has_no_self_cycled_nodes<G: Graph + ?Sized>(graph: &G) -> Result<(), String> {
    for node in graph.nodes() {
        let node_ptr = Arc::as_ptr(&node) as usize;
        let has_self = node
            .read()
            .unwrap()
            .nodes_from
            .iter()
            .any(|p| Arc::as_ptr(p) as usize == node_ptr);
        if has_self {
            return Err(format!("{ERROR_PREFIX} Graph has self-cycled nodes"));
        }
    }
    Ok(())
}

pub fn has_no_isolated_components<G: Graph + ?Sized>(graph: &G) -> Result<(), String> {
    if graph.length() == 0 {
        return Err(format!(
            "{ERROR_PREFIX} Graph is null, connectivity not defined"
        ));
    }

    let components = get_connected_components(graph, None);
    if components.len() > 1 {
        return Err(format!("{ERROR_PREFIX} Graph has isolated components"));
    }
    Ok(())
}
