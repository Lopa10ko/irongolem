//! elitism

#[test]
fn test_keep_n_best_elitism() {
    // def test_keep_n_best_elitism(set_up):
    //     best_individuals, population = set_up
    //     elitism = Elitism(GPAlgorithmParameters(elitism_type=ElitismTypesEnum.keep_n_best))
    //     new_population = elitism(best_individuals, population)
    //     for best_ind in best_individuals:
    //         # checks that new population contains the best individuals and `keep_n_best_elitism` does not duplicate it
    //         assert new_population.count(best_ind) == 1
    //     assert len(population) == len(new_population)
    assert!(false);
}

#[test]
fn test_replace_worst() {
    // def test_replace_worst(set_up):
    //     best_individuals, population = set_up
    //     elitism = Elitism(GPAlgorithmParameters(elitism_type=ElitismTypesEnum.replace_worst))
    //     new_population = elitism(best_individuals, population)
    //     for ind in population:
    //         if ind not in new_population:
    //             assert all(ind.fitness <= best_ind.fitness for best_ind in new_population)
    //     assert len(new_population) == len(population)
    assert!(false);
}

#[test]
fn test_elitism_not_applicable() {
    // def test_elitism_not_applicable(set_up):
    //     best_individuals, population = set_up
    //     elitisms = [
    //         Elitism(GPAlgorithmParameters(elitism_type=ElitismTypesEnum.replace_worst,
    //                                       multi_objective=True)),
    //         Elitism(GPAlgorithmParameters(elitism_type=ElitismTypesEnum.replace_worst,
    //                                       pop_size=4, min_pop_size_with_elitism=5)),
    //         Elitism(GPAlgorithmParameters(elitism_type=ElitismTypesEnum.none)),
    //     ]
    //     for elitism in elitisms:
    //         new_population = elitism(best_individuals, population)
    //         assert new_population == population
    assert!(false);
}
