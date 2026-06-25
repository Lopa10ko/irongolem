use super::fixtures::{graph_first, graph_fourth, graph_second, graph_third};

#[test]
fn test_equality_cases() {
    let pairs = [
        (graph_first(), graph_first()),
        (graph_third(), graph_third()),
        (graph_fourth(), graph_fourth()),
    ];
    for (left, right) in pairs {
        assert_eq!(left, right);
        assert_eq!(right, left);
    }
}

#[test]
fn test_non_equality_cases() {
    let pairs = [
        (graph_first(), graph_second()),
        (graph_first(), graph_third()),
        (graph_second(), graph_third()),
    ];
    for (left, right) in pairs {
        assert_ne!(left, right);
        assert_ne!(right, left);
    }
}
