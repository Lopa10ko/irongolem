//! Directed graph with an index-based arena representation.

mod graph;
pub mod graph_utils;
mod node;
mod reconnect;

pub use graph::{CycleError, LinkedGraph};
pub use node::{GraphNode, NodeContent, NodeId};
pub use reconnect::ReconnectType;
