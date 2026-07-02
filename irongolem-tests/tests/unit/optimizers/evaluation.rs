use std::sync::Arc;
use std::time::Duration;

use irongolem::golem::adapter::DirectAdapter;
use irongolem::golem::dag::GraphDelegate;
use irongolem::golem::optimisers::evaluation::{
    EvaluationDispatcher, MultiprocessingDispatcher, SequentialDispatcher,
};
use irongolem::golem::optimisers::fitness::{null_fitness, Fitness};
use irongolem::golem::optimisers::history::Individual;
use irongolem::golem::optimisers::objective::Objective;
use test_support::fixtures::{graph_first, graph_fourth, graph_second, graph_third, RandomMetric};
use test_support::golem::utilities::determine_n_jobs;

fn set_up_tests() -> (DirectAdapter, Vec<Individual>) {
    let adapter = DirectAdapter;
    let graphs = [graph_first(), graph_second(), graph_third(), graph_fourth()];
    let population: Vec<Individual> = graphs
        .into_iter()
        .map(|g| Individual::new(Arc::new(g)))
        .collect();
    (adapter, population)
}

fn get_objective(graph: Arc<GraphDelegate>) -> Fitness {
    let mut metrics = std::collections::HashMap::new();
    metrics.insert("random_metric".into(), "random".into());
    let objective = Objective::new(metrics);
    let _ = RandomMetric::get_value(graph.clone(), Duration::ZERO);
    objective.evaluate(graph)
}

fn invalid_objective(_graph: Arc<GraphDelegate>) -> Fitness {
    null_fitness()
}

#[test]
fn test_dispatchers_with_and_without_multiprocessing_sequential() {
    let (adapter, population) = set_up_tests();
    let dispatcher = SequentialDispatcher::new(adapter);
    let evaluator = dispatcher.dispatch(Arc::new(get_objective), None);
    let evaluated = evaluator(population.clone());
    assert!(evaluated.iter().all(|x| x.fitness.is_valid()));
    assert_eq!(population.len(), evaluated.len());
}

#[test]
fn test_dispatchers_with_and_without_multiprocessing_parallel() {
    let (adapter, population) = set_up_tests();
    let dispatcher = MultiprocessingDispatcher::new(adapter);
    let evaluator = dispatcher.dispatch(Arc::new(get_objective), None);
    let evaluated = evaluator(population.clone());
    assert!(evaluated.iter().all(|x| x.fitness.is_valid()));
    assert_eq!(population.len(), evaluated.len());
}

#[test]
fn test_dispatchers_with_and_without_multiprocessing_parallel_n_jobs() {
    let (adapter, population) = set_up_tests();
    let dispatcher = MultiprocessingDispatcher::with_n_jobs(adapter, -1);
    let evaluator = dispatcher.dispatch(Arc::new(get_objective), None);
    let evaluated = evaluator(population.clone());
    assert!(evaluated.iter().all(|x| x.fitness.is_valid()));
    assert_eq!(population.len(), evaluated.len());
}

#[test]
fn test_dispatchers_with_faulty_objectives_multiprocessing() {
    let (_adapter, population) = set_up_tests();
    let dispatcher = MultiprocessingDispatcher::new(DirectAdapter);
    let evaluator = dispatcher.dispatch(Arc::new(invalid_objective), None);
    assert!(evaluator(population).is_empty());
}

#[test]
fn test_dispatchers_with_faulty_objectives_sequential() {
    let (_adapter, population) = set_up_tests();
    let dispatcher = SequentialDispatcher::new(DirectAdapter);
    let evaluator = dispatcher.dispatch(Arc::new(invalid_objective), None);
    assert!(evaluator(population).is_empty());
}

#[test]
fn test_n_jobs_for_dispatcher() {
    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get() as i32)
        .unwrap_or(1);
    for n_jobs in -cpu_count..cpu_count + 5 {
        if n_jobs != 0 {
            let correct = if n_jobs > 0 {
                std::cmp::min(n_jobs as usize, cpu_count as usize)
            } else {
                (cpu_count + 1 + n_jobs) as usize
            };
            assert_eq!(determine_n_jobs(n_jobs).unwrap(), correct);
        }
    }
    for n_jobs in [0, -cpu_count - 1, -cpu_count - 2] {
        assert!(determine_n_jobs(n_jobs).is_err());
    }
}
