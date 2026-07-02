//! graph_verification

use irongolem::golem::dag::{
    has_no_cycle, has_no_isolated_components, has_no_isolated_nodes, has_no_self_cycled_nodes,
    ERROR_PREFIX,
};
use test_support::fixtures::{
    graph_first, graph_with_cycle, graph_with_cycled_node, graph_with_isolated_components,
    graph_with_isolated_nodes,
};

#[test]
fn test_graph_with_cycle_raise_exception() {
    let graph = graph_with_cycle();
    let err = has_no_cycle(&graph).unwrap_err();
    assert_eq!(err, format!("{ERROR_PREFIX} Graph has cycles"));
}

#[test]
fn test_graph_without_cycles_correct() {
    let graph = graph_first();
    assert!(has_no_cycle(&graph).is_ok());
}

#[test]
fn test_graph_with_isolated_nodes_raise_exception() {
    let graph = graph_with_isolated_nodes();
    let err = has_no_isolated_nodes(&graph).unwrap_err();
    assert_eq!(err, format!("{ERROR_PREFIX} Graph has isolated nodes"));
}

#[test]
fn test_graph_with_self_cycled_nodes_raise_exception() {
    let graph = graph_with_cycled_node();
    let err = has_no_self_cycled_nodes(&graph).unwrap_err();
    assert_eq!(err, format!("{ERROR_PREFIX} Graph has self-cycled nodes"));
}

#[test]
fn test_graph_with_isolated_components_raise_exception() {
    let graph = graph_with_isolated_components();
    let err = has_no_isolated_components(&graph).unwrap_err();
    assert_eq!(err, format!("{ERROR_PREFIX} Graph has isolated components"));
}
