use irongolem::golem::serializers::{dump_path_to_obj, CLASS_PATH_KEY};

#[test]
fn test_decoder() {
    let dumped = dump_path_to_obj("golem.core.optimisers.fitness.fitness", "Fitness");
    let class_path = dumped.get(CLASS_PATH_KEY).unwrap().as_str().unwrap();
    assert_eq!(class_path, "golem.core.optimisers.fitness.fitness/Fitness");
}
