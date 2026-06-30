//! fitness

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use irongolem::golem::optimisers::fitness::{
    fitness_objects, null_fitness, to_fitness, Fitness, MultiObjFitness, SingleObjFitness,
};

#[test]
fn test_fitness_hash() {
    let objects = fitness_objects();
    let hashes: HashSet<u64> = objects
        .iter()
        .map(|f| {
            let mut hasher = DefaultHasher::new();
            f.hash(&mut hasher);
            hasher.finish()
        })
        .collect();
    assert_eq!(hashes.len(), objects.len());
}

#[test]
fn test_fitness_values_property() {
    let mut fitness = Fitness::Multi(MultiObjFitness::new(&[1.0, 2.0], None));
    let fitness_values = vec![3.1415; fitness.values().len()];

    fitness.clear_values();
    assert!(!fitness.is_valid());

    fitness.set_values(fitness_values.clone());

    if !fitness_values.is_empty() {
        assert!(fitness.is_valid());
    }
    let weighted: Vec<f64> = fitness_values
        .iter()
        .zip(fitness.weights().iter())
        .map(|(v, w)| v * w)
        .collect();
    assert_eq!(fitness.values(), weighted);
}

#[test]
fn test_fitness_validity() {
    for fitness in fitness_objects() {
        let is_empty = fitness.values().is_empty();
        if is_empty {
            assert!(!fitness.is_valid());
        } else if matches!(&fitness, Fitness::Single(s) if !s.is_valid()) {
            assert!(!fitness.is_valid());
        } else {
            assert!(fitness.is_valid());
        }
    }
}

#[test]
fn test_fitness_invalid_are_unequal() {
    let objects = fitness_objects();
    for f1 in &objects {
        for f2 in &objects {
            if !f1.is_valid() || !f2.is_valid() {
                assert_ne!(f1, f2);
            }
        }
    }
}

#[test]
fn test_fitness_equality() {
    for fitness in fitness_objects() {
        let clone = match &fitness {
            Fitness::Single(s) => Fitness::Single(s.clone()),
            Fitness::Multi(m) => Fitness::Multi(m.clone()),
            Fitness::Invalid => Fitness::Invalid,
        };
        if fitness.is_valid() {
            assert_eq!(fitness, clone);
        } else {
            assert_ne!(fitness, clone);
        }
    }

    assert_eq!(
        Fitness::Single(SingleObjFitness::new(Some(1.0), &[2.0, 3.0 + 1e-12])),
        Fitness::Single(SingleObjFitness::new(Some(1.0), &[2.0, 3.0]))
    );
    assert_ne!(
        Fitness::Single(SingleObjFitness::new(Some(1.0), &[])),
        Fitness::Multi(MultiObjFitness::new(&[1.0], None))
    );
    assert_ne!(
        Fitness::Single(SingleObjFitness::new(Some(1.0), &[2.0, 3.0])),
        Fitness::Multi(MultiObjFitness::new(&[1.0, 2.0, 3.0], None))
    );
}

#[test]
fn test_fitness_compare_with_null_fitness() {
    assert!(Fitness::Single(SingleObjFitness::new(Some(1.0), &[10.0])) > null_fitness());
    assert_ne!(
        Fitness::Single(SingleObjFitness::new(None, &[10.0])),
        null_fitness()
    );
    assert!(Fitness::Multi(MultiObjFitness::new(&[1.0, 123.0, 123.0], None)) > null_fitness());
    assert!(Fitness::Multi(MultiObjFitness::new(&[0.0, 0.0, 0.0], None)) > null_fitness());
}

#[test]
fn test_fitness_compare_prioritised_invalid() {
    assert!(
        Fitness::Single(SingleObjFitness::new(None, &[10.0]))
            < Fitness::Single(SingleObjFitness::new(Some(1.0), &[20.0]))
    );
    assert!(
        Fitness::Single(SingleObjFitness::new(Some(1.0), &[10.0]))
            > Fitness::Single(SingleObjFitness::new(None, &[20.0]))
    );
    assert!(
        Fitness::Single(SingleObjFitness::new(None, &[123.0]))
            < Fitness::Single(SingleObjFitness::new(None, &[]))
    );
}

#[test]
fn test_fitness_compare_prioritised() {
    assert!(
        Fitness::Single(SingleObjFitness::new(Some(1.0), &[10.0]))
            > Fitness::Single(SingleObjFitness::new(Some(1.0), &[20.0]))
    );
    assert!(
        Fitness::Single(SingleObjFitness::new(Some(1.0), &[10.0, 100.0]))
            > Fitness::Single(SingleObjFitness::new(Some(1.0), &[10.0, 101.0]))
    );
    assert!(
        Fitness::Single(SingleObjFitness::new(Some(0.0), &[20.0]))
            > Fitness::Single(SingleObjFitness::new(Some(1.0), &[10.0]))
    );
}

#[test]
fn test_fitness_multiobj_dominates() {
    let m = |v: &[f64]| MultiObjFitness::new(v, None);
    assert!(m(&[1.0]).dominates(&m(&[2.0])));
    assert!(m(&[1.0, 1.0, 1.0]).dominates(&m(&[2.0, 2.0, 2.0])));
    assert!(m(&[1.0, 1.0, 3.0]).dominates(&m(&[1.0, 2.0, 3.0])));

    assert!(m(&[1.0]).dominates(&MultiObjFitness::new(&[1.0], Some(&[2.0]))));
    assert!(m(&[1.0, 1.0, 1.0]).dominates(&MultiObjFitness::new(&[1.0, 1.0, 1.0], Some(&[2.0]))));

    assert!(!m(&[1.0, 1.0, 1.0]).dominates(&m(&[1.0, 1.0, 1.0])));
    assert!(!m(&[1.0, 1.0, 2.0]).dominates(&m(&[1.0, 2.0, 1.0])));
}

#[test]
fn test_fitness_serialization() {
    for fitness in fitness_objects() {
        if !fitness.is_valid() {
            continue;
        }
        let dumped = serde_json::to_string(&fitness).unwrap();
        let reserialized: Fitness = serde_json::from_str(&dumped).unwrap();
        assert_eq!(fitness.values(), reserialized.values());
        assert_eq!(fitness.is_valid(), reserialized.is_valid());
        if fitness.is_valid() {
            assert_eq!(fitness, reserialized);
        }
    }
}

#[test]
fn test_universal_fitness_compare() {
    assert!(to_fitness(&[1.0, 1.0, 3.0], false).dominates(&to_fitness(&[1.0, 2.0, 3.0], false)));
    assert!(to_fitness(&[1.0, 1.0, 3.0], true).dominates(&to_fitness(&[1.0, 2.0, 3.0], true)));

    assert!(to_fitness(&[1.0, 1.0, 3.0], false).dominates(&to_fitness(&[1.0, 2.0, 1.0], false)));
    assert!(!to_fitness(&[1.0, 1.0, 3.0], true).dominates(&to_fitness(&[1.0, 2.0, 1.0], true)));
}
