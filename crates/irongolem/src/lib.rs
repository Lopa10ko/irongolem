//! IronGolem — evolutionary graph optimization in Rust.
//!
//! Rust reimplementation of GOLEM (Graph Optimiser for Learning and Evolution of Models).

pub mod golem;

pub use golem::adapter::DirectAdapter;
pub use golem::dag::{
    Graph, GraphDelegate, GraphImpl, GraphNode, LinkedGraph, LinkedGraphNode, NodeContent,
    ReconnectType,
};
pub use golem::optimisers::evaluation::{
    EvaluationDispatcher, MultiprocessingDispatcher, SequentialDispatcher, SurrogateDispatcher,
};
pub use golem::optimisers::fitness::{null_fitness, Fitness};
pub use golem::optimisers::history::Individual;
