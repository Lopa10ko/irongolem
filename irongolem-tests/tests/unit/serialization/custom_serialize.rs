use irongolem::golem::optimisers::fitness::Fitness;
use irongolem::golem::serializers::{default_load, default_save, encode_value, SerializerError};

#[test]
fn test_serializable() {
    let obj = Fitness::valid_fitness();
    let dumped = default_save(&obj, None).expect("save");
    let loaded: Fitness = default_load(&dumped).expect("load");
    assert_eq!(loaded, obj);
}

#[test]
fn test_default_save_load() {
    let obj = Fitness::valid_fitness();
    let json = default_save(&obj, None).expect("save");
    let loaded: Fitness = default_load(&json).expect("load");
    assert_eq!(loaded, obj);
}

#[test]
fn test_serializable_with_class_methods() {
    let obj = Fitness::valid_fitness();
    let dumped = default_save(&obj, None).expect("save");
    let loaded: Fitness = default_load(&dumped).expect("load");
    assert_eq!(loaded, obj);
}

#[test]
fn test_serializable_custom() {
    let obj = Fitness::valid_fitness();
    let dumped = default_save(&obj, None).expect("save");
    let loaded: Fitness = default_load(&dumped).expect("load");
    assert_eq!(loaded, obj);
}

#[test]
fn test_encode_value_rejects_non_object() {
    let err = encode_value(&42u32).unwrap_err();
    assert!(matches!(err, SerializerError::ExpectedObject));
}

#[test]
fn test_default_load_propagates_json_error() {
    let result: Result<Fitness, _> = default_load("not json");
    assert!(matches!(result, Err(SerializerError::Json(_))));
}
