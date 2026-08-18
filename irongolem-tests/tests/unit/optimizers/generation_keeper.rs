use std::thread;
use std::time::Duration;

use irongolem::golem::optimisers::timer::OptimisationTimer;
use test_support::fixtures::{generation_keeper, population1, population2};

#[test]
fn test_archive_no_improvement() {
    let mut archive = generation_keeper(population1());
    assert_eq!(archive.stagnation_iter_count(), 0);
    assert!(archive.is_any_improved());
    assert!(archive.is_quality_improved() && archive.is_complexity_improved());
    assert_eq!(archive.generation_num(), 1);

    archive.append(&population1());
    assert_eq!(archive.stagnation_iter_count(), 1);
    assert!(!archive.is_any_improved());
    assert!(!archive.is_quality_improved() && !archive.is_complexity_improved());
    assert_eq!(archive.generation_num(), 2);
}

#[test]
fn test_archive_multiobj_one_improvement() {
    let mut archive = generation_keeper(population1());
    let previous_size = archive.best_individuals().len();

    assert!(population2()
        .iter()
        .any(|new_ind| new_ind.fitness.dominates(&population1()[1].fitness)));
    archive.append(&population2());

    assert_eq!(archive.stagnation_iter_count(), 0);
    assert!(archive.is_any_improved());
    assert_eq!(archive.generation_num(), 2);
    assert_eq!(archive.best_individuals().len(), previous_size + 1);
    assert!(archive.is_complexity_improved());
    assert!(!archive.is_quality_improved());
}

#[test]
fn test_composition_timer() {
    let generation_num = 100usize;
    let mut reached = false;
    let start = std::time::Instant::now();
    let mut timer = OptimisationTimer::new(Duration::from_secs_f64(0.01 * 60.0));
    timer.start();
    for generation in 0..generation_num {
        thread::sleep(Duration::from_secs(1));
        if timer.is_time_limit_reached(Some(generation)) {
            reached = true;
            break;
        }
    }
    let spent_time = start.elapsed().as_secs();
    assert!(reached && spent_time == 1);
}
