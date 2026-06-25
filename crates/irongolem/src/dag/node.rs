use std::collections::BTreeMap;

use serde_json::Value;
use uuid::Uuid;

/// Stable handle to a node inside a [`LinkedGraph`](crate::dag::LinkedGraph) arena.
///
/// The id is an index into the graph's backing vector. It stays valid for the
/// lifetime of the graph even after the node is removed from the active node set,
/// so callers can keep references across mutations.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct NodeId(pub usize);

/// Content stored in a node: an operation `name` plus optional `params`.
///
/// `params` uses a `BTreeMap` so that [`GraphNode::description`] renders in a
/// deterministic order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeContent {
    pub name: String,
    pub params: BTreeMap<String, Value>,
}

impl NodeContent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            params: BTreeMap::new(),
        }
    }

    pub fn with_params(name: impl Into<String>, params: BTreeMap<String, Value>) -> Self {
        Self {
            name: name.into(),
            params,
        }
    }
}

impl From<&str> for NodeContent {
    fn from(name: &str) -> Self {
        NodeContent::new(name)
    }
}

impl From<String> for NodeContent {
    fn from(name: String) -> Self {
        NodeContent::new(name)
    }
}

/// A node in the directed graph.
#[derive(Clone, Debug)]
pub struct GraphNode {
    /// Globally unique identifier of the node.
    pub uid: Uuid,
    /// Operation name and parameters held by the node.
    pub content: NodeContent,
    /// Parent nodes, kept free of duplicates.
    pub parents: Vec<NodeId>,
}

impl GraphNode {
    pub(crate) fn new(content: NodeContent, parents: Vec<NodeId>) -> Self {
        Self {
            uid: Uuid::new_v4(),
            content,
            parents,
        }
    }

    pub fn name(&self) -> &str {
        &self.content.name
    }

    pub fn params(&self) -> &BTreeMap<String, Value> {
        &self.content.params
    }

    /// Returns the short label used inside the recursive descriptive id.
    ///
    /// The label is `n_<name>`, or `n_<name>_<params>` when parameters are
    /// present. The uid is used in place of an empty name.
    pub fn description(&self) -> String {
        let label = if self.content.name.is_empty() {
            self.uid.to_string()
        } else {
            self.content.name.clone()
        };
        if self.content.params.is_empty() {
            format!("n_{label}")
        } else {
            format!("n_{label}_{:?}", self.content.params)
        }
    }
}
