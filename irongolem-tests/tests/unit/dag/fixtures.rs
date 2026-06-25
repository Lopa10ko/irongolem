//! Graph fixtures shared by the dag tests, built on the `irongolem` arena graph.
//!
//! Nodes are created detached and then attached by calling `add_node` on the
//! root, which inserts them in pre-order. This keeps `graph.nodes()[i]` aligned
//! with the indices the tests rely on.

#![allow(dead_code)]

use irongolem::dag::LinkedGraph;

/// The 5-node fixture used by `graph_operator` / `graph_utils`:
/// `l0_n1 -> l1_n1 -> {l2_n1 -> l3_n1, l2_n2}`.
pub fn graph() -> LinkedGraph {
    let mut g = LinkedGraph::new();
    let l3 = g.add_detached("l3_n1", &[]);
    let l2_1 = g.add_detached("l2_n1", &[l3]);
    let l2_2 = g.add_detached("l2_n2", &[]);
    let l1 = g.add_detached("l1_n1", &[l2_1, l2_2]);
    let l0 = g.add_detached("l0_n1", &[l1]);
    g.add_node(l0);
    g
}

pub fn graph_first() -> LinkedGraph {
    //    a
    //  |     \
    //  a       b
    // |  \    |  \
    // c   d   c   d
    let mut g = LinkedGraph::new();
    let root = g.add_detached("a", &[]);
    let child_first = g.add_detached("a", &[]);
    let child_second = g.add_detached("b", &[]);

    for child in [child_first, child_second] {
        for oper in ["c", "d"] {
            let new_node = g.add_detached(oper, &[]);
            g.connect_nodes(new_node, child);
            g.add_node(new_node);
        }
        g.add_node(child);
        g.connect_nodes(child, root);
    }
    g.add_node(root);
    g
}

pub fn graph_second() -> LinkedGraph {
    //      a
    //   |      \
    //   a        b
    //  |  \     |  \
    //  c   a    c    d
    //     |  \
    //     b   d
    let mut g = LinkedGraph::new();
    let inner_b = g.add_detached("b", &[]);
    let inner_d = g.add_detached("d", &[]);
    let inner_a = g.add_detached("a", &[inner_b, inner_d]);
    let left_c = g.add_detached("c", &[]);
    let left_a = g.add_detached("a", &[left_c, inner_a]);
    let right_c = g.add_detached("c", &[]);
    let right_d = g.add_detached("d", &[]);
    let right_b = g.add_detached("b", &[right_c, right_d]);
    let root = g.add_detached("a", &[left_a, right_b]);
    g.add_node(root);
    g
}

pub fn graph_third() -> LinkedGraph {
    //      a
    //   /  |  \
    //  b   d   b
    let mut g = LinkedGraph::new();
    let root = g.add_detached("a", &[]);
    let b = g.add_detached("b", &[]);
    let d = g.add_detached("d", &[]);
    let b2 = g.add_detached("b", &[]);
    for child in [b, d, b2] {
        g.connect_nodes(child, root);
        g.add_node(child);
    }
    g.add_node(root);
    g
}

pub fn graph_fourth() -> LinkedGraph {
    //      a
    //   |  \  \
    //  b   a   b
    //      |  \
    //      b   b
    let mut g = LinkedGraph::new();
    let b = g.add_detached("b", &[]);
    let inner_b1 = g.add_detached("b", &[]);
    let inner_b2 = g.add_detached("b", &[]);
    let inner_a = g.add_detached("a", &[inner_b1, inner_b2]);
    let b3 = g.add_detached("b", &[]);
    let root = g.add_detached("a", &[b, inner_a, b3]);
    g.add_node(root);
    g
}

pub fn graph_fifth() -> LinkedGraph {
    // a
    // |
    // f   b
    //  \ /
    //   c
    //   |
    //   d
    //   |
    //   e
    let mut g = LinkedGraph::new();
    let a = g.add_detached("a", &[]);
    let f = g.add_detached("f", &[a]);
    let b = g.add_detached("b", &[]);
    let c = g.add_detached("c", &[f, b]);
    let d = g.add_detached("d", &[c]);
    let e = g.add_detached("e", &[d]);
    g.add_node(e);
    g
}

pub fn graph_with_multi_roots_first() -> LinkedGraph {
    //   17   16
    //   |  /    \
    //   15       14
    //     \      |  \
    //      13    12  11
    let mut g = LinkedGraph::new();
    let node1 = g.add_detached("11", &[]);
    let node2 = g.add_detached("12", &[]);
    let node3 = g.add_detached("13", &[]);
    let node4 = g.add_detached("14", &[node1, node2]);
    let node5 = g.add_detached("15", &[node3]);
    let node6 = g.add_detached("16", &[node4, node5]);
    let node7 = g.add_detached("17", &[node5]);
    g.add_node(node6);
    g.add_node(node7);
    g
}

pub fn joined_branches_graph() -> LinkedGraph {
    //   a
    //  / \
    // c - b
    // |   /
    // d  /
    // | /
    // f
    let mut g = LinkedGraph::new();
    let a = g.add_detached("a", &[]);
    let b = g.add_detached("b", &[a]);
    let c = g.add_detached("c", &[b, a]);
    let d = g.add_detached("d", &[c]);
    let f = g.add_detached("f", &[d, b]);
    g.add_node(f);
    g
}

pub fn simple_cycled_graph() -> LinkedGraph {
    let mut g = LinkedGraph::new();
    let a = g.add_detached("a", &[]);
    let b = g.add_detached("b", &[a]);
    let c = g.add_detached("c", &[b]);
    let d = g.add_detached("d", &[c]);
    let e = g.add_detached("e", &[d]);
    g.connect_nodes(e, b);
    g.add_node(d);
    g
}

pub fn branched_cycled_graph() -> LinkedGraph {
    let mut g = LinkedGraph::new();
    let a = g.add_detached("a", &[]);
    let b = g.add_detached("b", &[a]);
    let c = g.add_detached("c", &[b]);
    let d = g.add_detached("d", &[c]);
    let e = g.add_detached("e", &[d]);
    g.connect_nodes(e, b);

    let f = g.add_detached("f", &[a]);
    let _g_node = g.add_detached("g", &[f]);
    let h = g.add_detached("h", &[f]);
    g.add_node(d);
    g.add_node(_g_node);
    g.add_node(h);
    g
}

/// Base graph used by the disconnect tests.
pub fn get_initial_graph() -> LinkedGraph {
    let mut g = LinkedGraph::new();
    let a = g.add_detached("a", &[]);
    let b = g.add_detached("b", &[a]);
    let c = g.add_detached("c", &[a]);
    let c_second = g.add_detached("c", &[a]);
    let d = g.add_detached("d", &[c_second]);
    let e = g.add_detached("e", &[b, c]);
    let e_root = g.add_detached("e", &[d, e]);
    g.add_node(e_root);
    g
}

pub fn res_graph_test_first() -> LinkedGraph {
    let mut g = LinkedGraph::new();
    let a = g.add_detached("a", &[]);
    let c_second = g.add_detached("c", &[a]);
    let d = g.add_detached("d", &[c_second]);
    let e_root = g.add_detached("e", &[d]);
    g.add_node(e_root);
    g
}

pub fn res_graph_test_second() -> LinkedGraph {
    let mut g = LinkedGraph::new();
    let a = g.add_detached("a", &[]);
    let c = g.add_detached("c", &[a]);
    let c_second = g.add_detached("c", &[a]);
    let d = g.add_detached("d", &[c_second]);
    let e = g.add_detached("e", &[c]);
    let e_root = g.add_detached("e", &[d, e]);
    g.add_node(e_root);
    g
}

pub fn res_graph_test_third() -> LinkedGraph {
    let mut g = LinkedGraph::new();
    let a = g.add_detached("a", &[]);
    let b = g.add_detached("b", &[a]);
    let c = g.add_detached("c", &[a]);
    let e = g.add_detached("e", &[b, c]);
    let e_root = g.add_detached("e", &[e]);
    g.add_node(e_root);
    g
}
