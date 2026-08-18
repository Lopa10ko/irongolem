use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, RwLock};

use serde_json::Value;

use super::graph_node::GraphNode;

#[derive(Debug, Clone)]
pub struct NodeContent {
    pub name: String,
    pub params: BTreeMap<String, Value>,
    /// Extra content keys (Python `important_field`, `matrix`, metadata, …).
    pub extra: BTreeMap<String, Value>,
}

impl NodeContent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            params: BTreeMap::new(),
            extra: BTreeMap::new(),
        }
    }

    pub fn with_params(name: impl Into<String>, params: BTreeMap<String, Value>) -> Self {
        Self {
            name: name.into(),
            params,
            extra: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct LinkedGraphNode {
    pub uid: String,
    pub content: NodeContent,
    pub nodes_from: Vec<Arc<RwLock<LinkedGraphNode>>>,
}

impl LinkedGraphNode {
    pub fn new(content: NodeContent) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            uid: uuid::Uuid::new_v4().to_string(),
            content,
            nodes_from: Vec::new(),
        }))
    }

    pub fn with_parents(
        content: impl Into<NodeContent>,
        parents: Vec<Arc<RwLock<LinkedGraphNode>>>,
    ) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            uid: uuid::Uuid::new_v4().to_string(),
            content: content.into(),
            nodes_from: parents,
        }))
    }

    pub fn from_name(name: &str) -> Arc<RwLock<Self>> {
        Self::new(NodeContent::new(name))
    }

    pub fn with_uid(uid: impl Into<String>, content: NodeContent) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            uid: uid.into(),
            content,
            nodes_from: Vec::new(),
        }))
    }
}

impl From<&str> for NodeContent {
    fn from(s: &str) -> Self {
        NodeContent::new(s)
    }
}

impl GraphNode for LinkedGraphNode {
    fn descriptive_id(&self) -> String {
        super::graph_node::descriptive_id(self)
    }

    fn description(&self) -> String {
        let label = if self.content.name.is_empty() {
            self.uid.as_str()
        } else {
            self.content.name.as_str()
        };
        if self.content.params.is_empty() {
            format!("n_{label}")
        } else {
            format!("n_{label}_{:?}", self.content.params)
        }
    }

    fn nodes_from(&self) -> &[Arc<RwLock<LinkedGraphNode>>] {
        &self.nodes_from
    }
}

impl fmt::Display for LinkedGraphNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content.name)
    }
}
