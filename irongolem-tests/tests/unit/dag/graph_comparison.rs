//! graph_comparison

use test_support::fixtures::{equality_cases, non_equality_cases};

#[test]
fn test_equality_cases() {
    for (left, right) in equality_cases() {
        assert_eq!(left, right);
        assert_eq!(right, left);
    }
}

#[test]
fn test_non_equality_cases() {
    for (left, right) in non_equality_cases() {
        assert_ne!(left, right);
        assert_ne!(right, left);
    }
}
