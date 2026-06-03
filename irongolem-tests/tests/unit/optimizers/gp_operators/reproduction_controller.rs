//! reproduction_controller

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
    assert!(false);
}

#[test]
fn test_too_little_valid_evals() {
    // def test_too_little_valid_evals(reproducer: ReproductionController, success_rate: float):
    //     evaluator = MockEvaluator(success_rate)
    //     pop = get_rand_population(reproducer.parameters.pop_size)
    //
    //     with pytest.raises(EvaluationAttemptsError):
    //         reproducer.reproduce(pop, evaluator)
    assert!(false);
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
    assert!(false);
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
    assert!(false);
}
