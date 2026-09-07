use irongolem::golem::serializers::{dump_path_to_obj, CLASS_PATH_KEY};

#[test]
fn test_encoder() {
    let path = dump_path_to_obj("golem.core.optimisers.fitness.fitness", "Fitness");
    assert!(path.contains_key(CLASS_PATH_KEY));
}
