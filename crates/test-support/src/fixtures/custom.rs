use std::sync::{Arc, RwLock};

use irongolem::golem::adapter::DirectAdapter;
use irongolem::golem::dag::{Graph, GraphDelegate, LinkedGraphNode};
use irongolem::golem::optimisers::objective::Objective;

use super::graphs::{graph_fifth, graph_first, graph_fourth, graph_second, graph_third};

#[derive(Debug, Clone)]
pub struct CustomNode(pub Arc<RwLock<LinkedGraphNode>>);

#[derive(Debug, Clone)]
pub struct CustomModel {
    pub graph: GraphDelegate,
}

impl CustomModel {
    pub fn evaluate(&self) -> f64 {
        0.0
    }

    pub fn nodes(&self) -> Vec<CustomNode> {
        self.graph.nodes().into_iter().map(CustomNode).collect()
    }

    pub fn length(&self) -> usize {
        self.graph.length()
    }
}

pub struct CustomDirectAdapter;

impl CustomDirectAdapter {
    pub fn adapt(&self, model: CustomModel) -> Arc<GraphDelegate> {
        Arc::new(model.graph)
    }

    pub fn restore(&self, graph: Arc<GraphDelegate>) -> CustomModel {
        CustomModel {
            graph: DirectAdapter.restore(graph),
        }
    }
}

pub fn custom_metric(graph: Arc<GraphDelegate>) -> f64 {
    let g = graph.as_ref();
    -(g.length() as f64) + CustomModel { graph: g.clone() }.evaluate()
}

pub fn custom_objective() -> Objective {
    let mut metrics = std::collections::HashMap::new();
    metrics.insert("custom".into(), "custom".into());
    Objective::new(metrics).with_evaluator("custom", Arc::new(custom_metric))
}

pub fn custom_initial_graphs() -> Vec<CustomModel> {
    [
        graph_first(),
        graph_second(),
        graph_third(),
        graph_fourth(),
        graph_fifth(),
    ]
    .into_iter()
    .map(|g| CustomModel { graph: g })
    .collect()
}
