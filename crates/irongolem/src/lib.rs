//! Evolutionary graph optimization in Rust.
//!
//! The crate is built up module by module. The first available module is [`dag`],
//! a directed graph with an index-based arena representation chosen for
//! cache-friendly storage, cheap cloning, and `Send + Sync` access without locks.
//!
//! Diagnostic output is handled by [`logging`], which writes to a configurable
//! log file instead of the console.

pub mod dag;
pub mod logging;

/// Commonly used graph types, re-exported for convenient glob imports.
pub mod prelude {
    pub use crate::dag::{GraphNode, LinkedGraph, NodeContent, NodeId, ReconnectType};
}
