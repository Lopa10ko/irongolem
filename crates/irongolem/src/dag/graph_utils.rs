use super::graph::{CycleError, LinkedGraph};
use super::node::NodeId;

pub fn graph_has_cycle(graph: &LinkedGraph) -> bool {
    graph.graph_has_cycle()
}

pub fn ordered_subnodes_hierarchy(
    graph: &LinkedGraph,
    node: NodeId,
) -> Result<Vec<NodeId>, CycleError> {
    graph.ordered_subnodes_hierarchy(node)
}

pub fn node_depth(graph: &LinkedGraph, nodes: &[NodeId]) -> i64 {
    graph.node_depth_of(nodes)
}

pub fn distance_to_primary_level(graph: &LinkedGraph, node: NodeId) -> i64 {
    graph.distance_to_primary_level(node)
}

pub fn distance_to_root_level(graph: &LinkedGraph, node: NodeId) -> i64 {
    graph.distance_to_root_level(node)
}

pub fn nodes_from_layer(graph: &LinkedGraph, layer: i64) -> Vec<NodeId> {
    graph.nodes_from_layer(layer)
}
