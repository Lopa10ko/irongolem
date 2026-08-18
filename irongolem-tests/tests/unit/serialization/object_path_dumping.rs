use irongolem::golem::serializers::{dump_path_to_obj, CLASS_PATH_KEY};

#[test]
fn test_dumping() {
    let path = dump_path_to_obj("golem.core.dag.linked_graph_node", "LinkedGraphNode");
    assert!(path.get(CLASS_PATH_KEY).is_some());
}
