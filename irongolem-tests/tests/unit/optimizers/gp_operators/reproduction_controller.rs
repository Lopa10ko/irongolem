//! reproduction_controller

use irongolem::golem::optimisers::genetic::operators::reproduction::EvaluationAttemptsError;
use irongolem::golem::optimisers::genetic::parameters::parameter::AdaptiveParameter;
use irongolem::golem::optimisers::genetic::parameters::population_size::ConstRatePopulationSize;
use irongolem::golem::optimisers::genetic::rng::set_random_seed;
use test_support::fixtures::{get_rand_population, is_close, mock_evaluator, reproducer_fixture};

#[test]
fn test_mean_success_rate() {
    // def test_mean_success_rate(reproducer: ReproductionController, success_rate: float):
    //     """Tests that Reproducer correctly estimates average success rate"""
    //     assert np.isclose(reproducer.mean_success_rate, 1.0)
    //
    //     evaluator = MockEvaluator(success_rate)
    //     pop = get_rand_population(reproducer.parameters.pop_size)
    //     num_iters = 50
    //     for i in range(num_iters):
    //         pop = reproducer.reproduce(pop, evaluator)
    //
    //     assert np.isclose(reproducer.mean_success_rate, success_rate, rtol=0.1)
    let success_rate = 0.5;
    set_random_seed(42);
    let reproducer = reproducer_fixture();
    assert!(is_close(reproducer.mean_success_rate(), 1.0, 0.1));

    let evaluator = mock_evaluator(success_rate);
    let mut pop = get_rand_population(reproducer.parameters.pop_size);
    for _ in 0..50 {
        pop = reproducer.reproduce(pop, &evaluator).unwrap();
    }
    let estimated = reproducer.mean_success_rate();
    assert!(
        is_close(estimated, success_rate, 0.1),
        "mean_success_rate={estimated} expected={success_rate}"
    );
}

#[test]
fn test_too_little_valid_evals() {
    // def test_too_little_valid_evals(reproducer: ReproductionController, success_rate: float):
    //     evaluator = MockEvaluator(success_rate)
    //     pop = get_rand_population(reproducer.parameters.pop_size)
    //
    //     with pytest.raises(EvaluationAttemptsError):
    //         reproducer.reproduce(pop, evaluator)
    let success_rate = 0.0;
    let reproducer = reproducer_fixture();
    let evaluator = mock_evaluator(success_rate);
    let pop = get_rand_population(reproducer.parameters.pop_size);
    let result = reproducer.reproduce(pop, &evaluator);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        EvaluationAttemptsError { .. }
    ));
}

#[test]
fn test_minimal_valid_evals() {
    // def test_minimal_valid_evals(reproducer: ReproductionController, success_rate: float):
    //     parameters = reproducer.parameters
    //     evaluator = MockEvaluator(success_rate)
    //     pop = get_rand_population(parameters.pop_size)
    //     num_iters = 10
    //     for i in range(num_iters):
    //         pop = reproducer.reproduce(pop, evaluator)
    //         actual_valid_ratio = len(pop) / parameters.pop_size
    //         assert parameters.required_valid_ratio > actual_valid_ratio >= reproducer._minimum_valid_ratio
    let success_rate = 0.2;
    let reproducer = reproducer_fixture();
    let evaluator = mock_evaluator(success_rate);
    let mut pop = get_rand_population(reproducer.parameters.pop_size);
    for _ in 0..10 {
        pop = reproducer.reproduce(pop, &evaluator).unwrap();
        let actual_valid_ratio = pop.len() as f64 / reproducer.parameters.pop_size as f64;
        assert!(reproducer.parameters.required_valid_ratio > actual_valid_ratio);
        assert!(actual_valid_ratio >= reproducer.minimum_valid_ratio());
    }
}

#[test]
fn test_pop_size_progression() {
    // def test_pop_size_progression(reproducer: ReproductionController, success_rate: float):
    //     parameters = reproducer.parameters
    //     required_valid = parameters.required_valid_ratio
    //     pop_size_progress = ConstRatePopulationSize(parameters.pop_size,
    //                                                 parameters.offspring_rate,
    //                                                 parameters.max_pop_size)
    //
    //     evaluator = MockEvaluator(success_rate)
    //     pop = get_rand_population(parameters.pop_size)
    //     num_iters = 50
    //     for i in range(num_iters):
    //         prev_pop = pop
    //         pop = reproducer.reproduce(pop, evaluator)
    //         actual_pop_size = len(pop)
    //
    //         # test that even with noisy evaluators we have steady increase in offsprings
    //         if i > 1:
    //             assert (actual_pop_size > len(prev_pop) or
    //                     actual_pop_size >= parameters.max_pop_size * required_valid)
    //         # and that this increase follows the one from parameters
    //         assert 1.0 >= (actual_pop_size / parameters.pop_size) >= required_valid
    //
    //         # update pop size
    //         parameters.pop_size = pop_size_progress.next(pop)
    let success_rate = 0.5;
    let mut reproducer = reproducer_fixture();
    let required_valid = reproducer.parameters.required_valid_ratio;
    let max_pop_size = reproducer.parameters.max_pop_size.unwrap_or(100);
    let pop_size_progress = ConstRatePopulationSize::new(
        reproducer.parameters.pop_size,
        reproducer.parameters.offspring_rate,
        Some(max_pop_size),
    );
    let evaluator = mock_evaluator(success_rate);
    let mut pop = get_rand_population(reproducer.parameters.pop_size);
    for i in 0..50 {
        let prev_pop = pop.clone();
        pop = reproducer.reproduce(pop, &evaluator).unwrap();
        let actual_pop_size = pop.len();
        let parameters_pop_size = reproducer.parameters.pop_size;

        if i > 1 {
            assert!(
                actual_pop_size > prev_pop.len()
                    || actual_pop_size >= ((max_pop_size as f64 * required_valid).ceil() as usize)
            );
        }
        let ratio = actual_pop_size as f64 / parameters_pop_size as f64;
        assert!(ratio <= 1.0);
        assert!(ratio >= required_valid);

        reproducer.parameters.pop_size = pop_size_progress.next(&pop);
    }
}
