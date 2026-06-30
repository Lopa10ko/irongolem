//! elitism

use irongolem::golem::optimisers::genetic::operators::elitism::{Elitism, ElitismTypesEnum};
use irongolem::golem::optimisers::genetic::params::{GPAlgorithmParameters, GraphRequirements};
use test_support::fixtures::elitism_set_up;

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
    let (best_individuals, population) = elitism_set_up();
    let parameters =
        GPAlgorithmParameters::default().with_elitism_type(ElitismTypesEnum::KeepNBest);
    let elitism = Elitism::new(parameters, GraphRequirements::default());
    let new_population = elitism.call(best_individuals.clone(), population.clone());
    for best_ind in &best_individuals {
        let count = new_population
            .iter()
            .filter(|ind| ind.uid == best_ind.uid)
            .count();
        assert_eq!(count, 1);
    }
    assert_eq!(population.len(), new_population.len());
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
    let (best_individuals, population) = elitism_set_up();
    let parameters =
        GPAlgorithmParameters::default().with_elitism_type(ElitismTypesEnum::ReplaceWorst);
    let elitism = Elitism::new(parameters, GraphRequirements::default());
    let new_population = elitism.call(best_individuals, population.clone());
    for ind in &population {
        if !new_population.iter().any(|n| n.uid == ind.uid) {
            assert!(new_population
                .iter()
                .all(|best_ind| ind.fitness <= best_ind.fitness));
        }
    }
    assert_eq!(new_population.len(), population.len());
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
    let (best_individuals, population) = elitism_set_up();
    let elitisms = [
        GPAlgorithmParameters::default()
            .with_elitism_type(ElitismTypesEnum::ReplaceWorst)
            .with_multi_objective(true),
        {
            let mut params = GPAlgorithmParameters::new(4);
            params.min_pop_size_with_elitism = 5;
            params.elitism_type = ElitismTypesEnum::ReplaceWorst;
            params
        },
        GPAlgorithmParameters::default().with_elitism_type(ElitismTypesEnum::None),
    ];
    for parameters in elitisms {
        let elitism = Elitism::new(parameters, GraphRequirements::default());
        let new_population = elitism.call(best_individuals.clone(), population.clone());
        assert_eq!(new_population.len(), population.len());
        for (old, new) in population.iter().zip(new_population.iter()) {
            assert_eq!(old.uid, new.uid);
        }
    }
}
