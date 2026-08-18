use std::sync::Arc;
use std::time::Duration;

use irongolem::golem::adapter::DirectAdapter;
use irongolem::golem::dag::GraphDelegate;
use irongolem::golem::optimisers::evaluation::{
    EvaluationDispatcher, MultiprocessingDispatcher, SequentialDispatcher, SurrogateDispatcher,
};
use irongolem::golem::optimisers::fitness::{null_fitness, Fitness};
use irongolem::golem::optimisers::history::Individual;
use irongolem::golem::optimisers::objective::Objective;
use irongolem::golem::optimisers::timer::OptimisationTimer;
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

fn make_objective(delay: Duration) -> Arc<dyn Fn(Arc<GraphDelegate>) -> Fitness + Send + Sync> {
    Arc::new(move |graph| {
        let mut metrics = std::collections::HashMap::new();
        metrics.insert("random_metric".into(), "random".into());
        let objective = Objective::new(metrics);
        let _ = RandomMetric::get_value(graph.clone(), delay);
        objective.evaluate(graph)
    })
}

fn invalid_objective(_graph: Arc<GraphDelegate>) -> Fitness {
    null_fitness()
}

fn run_timeout_test<D: EvaluationDispatcher>(make_dispatcher: impl Fn() -> D, check_partial: bool) {
    let (_, population) = set_up_tests();

    let mut timer = OptimisationTimer::new(Duration::from_millis(10));
    timer.start();
    let dispatcher = make_dispatcher();
    let evaluator = dispatcher.dispatch(make_objective(Duration::from_millis(100)), Some(timer));
    let evaluated = evaluator(population.clone());
    assert!(
        evaluated.iter().all(|x| x.fitness.is_valid()),
        "At least one fitness value is invalid"
    );
    assert!(
        !evaluated.is_empty(),
        "At least one graph should be evaluated"
    );
    if check_partial {
        assert!(
            evaluated.len() < population.len(),
            "Not all graphs should be evaluated (not enough time)"
        );
    }

    let mut timer = OptimisationTimer::new(Duration::from_secs(5 * 60));
    timer.start();
    let dispatcher = make_dispatcher();
    let evaluator = dispatcher.dispatch(Arc::new(get_objective), Some(timer));
    let evaluated = evaluator(population);
    assert!(
        evaluated.iter().all(|x| x.fitness.is_valid()),
        "At least one fitness value is invalid"
    );
    assert_eq!(4, evaluated.len(), "Not all graphs were evaluated");
}

#[test]
fn test_dispatchers_with_and_without_multiprocessing_sequential() {
    let (adapter, population) = set_up_tests();
    let dispatcher = SequentialDispatcher::new(Arc::new(adapter));
    let evaluator = dispatcher.dispatch(Arc::new(get_objective), None);
    let evaluated = evaluator(population.clone());
    assert!(evaluated.iter().all(|x| x.fitness.is_valid()));
    assert_eq!(population.len(), evaluated.len());
}

#[test]
fn test_dispatchers_with_and_without_multiprocessing_parallel() {
    let (adapter, population) = set_up_tests();
    let dispatcher = MultiprocessingDispatcher::new(Arc::new(adapter));
    let evaluator = dispatcher.dispatch(Arc::new(get_objective), None);
    let evaluated = evaluator(population.clone());
    assert!(evaluated.iter().all(|x| x.fitness.is_valid()));
    assert_eq!(population.len(), evaluated.len());
}

#[test]
fn test_dispatchers_with_and_without_multiprocessing_parallel_n_jobs() {
    let (adapter, population) = set_up_tests();
    let dispatcher = MultiprocessingDispatcher::with_n_jobs(Arc::new(adapter), -1);
    let evaluator = dispatcher.dispatch(Arc::new(get_objective), None);
    let evaluated = evaluator(population.clone());
    assert!(evaluated.iter().all(|x| x.fitness.is_valid()));
    assert_eq!(population.len(), evaluated.len());
}

#[test]
fn test_dispatchers_with_faulty_objectives_multiprocessing() {
    let (_adapter, population) = set_up_tests();
    let dispatcher = MultiprocessingDispatcher::new(Arc::new(DirectAdapter));
    let evaluator = dispatcher.dispatch(Arc::new(invalid_objective), None);
    assert!(evaluator(population).is_empty());
}

#[test]
fn test_dispatchers_with_faulty_objectives_sequential() {
    let (_adapter, population) = set_up_tests();
    let dispatcher = SequentialDispatcher::new(Arc::new(DirectAdapter));
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

#[test]
fn test_dispatcher_with_timeout_sequential() {
    run_timeout_test(|| SequentialDispatcher::new(Arc::new(DirectAdapter)), true);
}

#[test]
fn test_dispatcher_with_timeout_multiprocessing() {
    run_timeout_test(
        || MultiprocessingDispatcher::new(Arc::new(DirectAdapter)),
        true,
    );
}

#[test]
fn test_dispatcher_with_timeout_surrogate() {
    run_timeout_test(|| SurrogateDispatcher::new(Arc::new(DirectAdapter)), false);
}
