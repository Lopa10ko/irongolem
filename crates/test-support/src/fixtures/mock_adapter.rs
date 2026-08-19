use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use irongolem::golem::adapter::{adapt_native_fn, OptimizationAdapter};
use irongolem::golem::dag::{Graph, GraphDelegate, LinkedGraphNode, NodeContent};

#[derive(Debug, Clone)]
pub struct MockNode {
    pub content: HashMap<String, serde_json::Value>,
    pub nodes_from: Vec<Arc<MockNode>>,
}

impl MockNode {
    pub fn new(name: &str) -> Self {
        let mut content = HashMap::new();
        content.insert("name".into(), serde_json::Value::String(name.into()));
        Self {
            content,
            nodes_from: Vec::new(),
        }
    }

    pub fn with_parent(name: &str, parent: MockNode) -> Self {
        Self::with_parents(name, vec![Arc::new(parent)])
    }

    pub fn with_parents(name: &str, parents: Vec<Arc<MockNode>>) -> Self {
        let mut node = Self::new(name);
        node.nodes_from = parents;
        node
    }
}

#[derive(Debug, Clone)]
pub struct MockDomainStructure {
    pub nodes: Vec<MockNode>,
}

impl MockDomainStructure {
    pub fn new(nodes: Vec<MockNode>) -> Self {
        Self { nodes }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockAdapter;

fn linked_from_content(
    mock: &MockNode,
    parents: Vec<Arc<RwLock<LinkedGraphNode>>>,
) -> Arc<RwLock<LinkedGraphNode>> {
    let name = mock
        .content
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("a");
    let mut content = NodeContent::new(name);
    if let Some(params) = mock.content.get("params").and_then(|v| v.as_object()) {
        for (k, v) in params {
            content.params.insert(k.clone(), v.clone());
        }
    }
    for (k, v) in &mock.content {
        if k != "name" && k != "params" {
            content.extra.insert(k.clone(), v.clone());
        }
    }
    if parents.is_empty() {
        LinkedGraphNode::new(content)
    } else {
        LinkedGraphNode::with_parents(content, parents)
    }
}

fn node_from_arc(
    mock: &Arc<MockNode>,
    cache: &mut HashMap<usize, Arc<RwLock<LinkedGraphNode>>>,
) -> Arc<RwLock<LinkedGraphNode>> {
    let ptr = Arc::as_ptr(mock) as usize;
    if let Some(existing) = cache.get(&ptr) {
        return existing.clone();
    }
    let parents: Vec<_> = mock
        .nodes_from
        .iter()
        .map(|parent| node_from_arc(parent, cache))
        .collect();
    let node = linked_from_content(mock.as_ref(), parents);
    cache.insert(ptr, node.clone());
    node
}

fn node_from_mock(
    mock: &MockNode,
    cache: &mut HashMap<usize, Arc<RwLock<LinkedGraphNode>>>,
) -> Arc<RwLock<LinkedGraphNode>> {
    let parents: Vec<_> = mock
        .nodes_from
        .iter()
        .map(|parent| node_from_arc(parent, cache))
        .collect();
    linked_from_content(mock, parents)
}

impl MockAdapter {
    fn to_graph(&self, domain: &MockDomainStructure) -> GraphDelegate {
        if domain.nodes.is_empty() {
            return GraphDelegate::empty();
        }
        let mut cache = HashMap::new();
        let arcs: Vec<_> = domain
            .nodes
            .iter()
            .map(|node| node_from_mock(node, &mut cache))
            .collect();
        GraphDelegate::with_roots(arcs)
    }

    fn to_domain(&self, graph: &GraphDelegate) -> MockDomainStructure {
        fn mock_from_opt(
            node: &Arc<RwLock<LinkedGraphNode>>,
            cache: &mut HashMap<usize, Arc<MockNode>>,
        ) -> Arc<MockNode> {
            let ptr = Arc::as_ptr(node) as usize;
            if let Some(cached) = cache.get(&ptr) {
                return cached.clone();
            }
            let guard = node.read().unwrap();
            let mut content = HashMap::from([(
                "name".into(),
                serde_json::Value::String(guard.content.name.clone()),
            )]);
            if !guard.content.params.is_empty() {
                let params: serde_json::Map<String, serde_json::Value> = guard
                    .content
                    .params
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                content.insert("params".into(), serde_json::Value::Object(params));
            }
            for (k, v) in &guard.content.extra {
                content.insert(k.clone(), v.clone());
            }
            let parents: Vec<_> = guard
                .nodes_from
                .iter()
                .map(|p| mock_from_opt(p, cache))
                .collect();
            drop(guard);
            let mock = Arc::new(MockNode {
                content,
                nodes_from: parents,
            });
            cache.insert(ptr, mock.clone());
            mock
        }

        let mut cache = HashMap::new();
        let nodes = graph
            .root_nodes()
            .into_iter()
            .map(|n| (*mock_from_opt(&n, &mut cache)).clone())
            .collect();
        MockDomainStructure { nodes }
    }

    pub fn adapt_func<A, R>(&self, fun: fn(&A) -> R) -> fn(&A) -> R {
        adapt_native_fn(fun)
    }

    pub fn restore_func<F>(&self, fun: F) -> F {
        fun
    }
}

impl OptimizationAdapter for MockAdapter {
    fn adapt_graph(&self, graph: GraphDelegate) -> Arc<GraphDelegate> {
        let domain = self.to_domain(&graph);
        Arc::new(self.to_graph(&domain))
    }

    fn restore_graph(&self, graph: Arc<GraphDelegate>) -> GraphDelegate {
        let domain = self.to_domain(graph.as_ref());
        self.to_graph(&domain)
    }
}

impl MockAdapter {
    pub fn adapt(&self, item: MockDomainStructure) -> Arc<GraphDelegate> {
        Arc::new(self.to_graph(&item))
    }

    pub fn restore(&self, graph: Arc<GraphDelegate>) -> MockDomainStructure {
        self.to_domain(graph.as_ref())
    }

    pub fn adapt_many(&self, graphs: Vec<MockDomainStructure>) -> Vec<Arc<GraphDelegate>> {
        graphs.into_iter().map(|g| self.adapt(g)).collect()
    }
}

pub fn graph_with_params(alpha: f64) -> MockDomainStructure {
    let mut node = MockNode::new("root");
    node.content
        .insert("params".into(), serde_json::json!({"alpha": alpha}));
    MockDomainStructure::new(vec![node])
}
