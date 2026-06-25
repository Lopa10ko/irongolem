//! Graph verification tests.
//!
//! These exercise the verification rules (`has_no_cycle`, `has_no_isolated_nodes`,
//! and similar), which are scheduled for the next iteration. The pure-graph
//! primitives they build on (`graph_has_cycle`, `node_children`, and others) are
//! already implemented and covered by the `graph_utils` tests, so these stay
//! ignored until the rules land.

#[test]
#[ignore = "verification rules: next iteration (verifier)"]
fn test_graph_with_cycle_raise_exception() {}

#[test]
#[ignore = "verification rules: next iteration (verifier)"]
fn test_graph_without_cycles_correct() {}

#[test]
#[ignore = "verification rules: next iteration (verifier)"]
fn test_graph_with_isolated_nodes_raise_exception() {}

#[test]
#[ignore = "verification rules: next iteration (verifier)"]
fn test_graph_with_self_cycled_nodes_raise_exception() {}

#[test]
#[ignore = "verification rules: next iteration (verifier)"]
fn test_graph_with_isolated_components_raise_exception() {}
