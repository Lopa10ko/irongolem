use irongolem::golem::optimisers::fitness::Fitness;
use irongolem::golem::serializers::{default_load, default_save};

#[test]
fn test_serializable() {
    let obj = Fitness::valid_fitness();
    let dumped = default_save(&obj, None);
    let loaded: Fitness = default_load(&dumped).unwrap();
    assert_eq!(loaded, obj);
}

#[test]
fn test_default_save_load() {
    let obj = Fitness::valid_fitness();
    let json = default_save(&obj, None);
    let loaded: Fitness = default_load(&json).unwrap();
    assert_eq!(loaded, obj);
}

#[test]
fn test_serializable_with_class_methods() {
    let obj = Fitness::valid_fitness();
    let dumped = default_save(&obj, None);
    let loaded: Fitness = default_load(&dumped).unwrap();
    assert_eq!(loaded, obj);
}

#[test]
fn test_serializable_custom() {
    let obj = Fitness::valid_fitness();
    let dumped = default_save(&obj, None);
    let loaded: Fitness = default_load(&dumped).unwrap();
    assert_eq!(loaded, obj);
}
