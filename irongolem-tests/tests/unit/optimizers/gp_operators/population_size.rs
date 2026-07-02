//! population_size

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use irongolem::golem::dag::GraphDelegate;
use irongolem::golem::dag::LinkedGraphNode;
use irongolem::golem::optimisers::fitness::{Fitness, SingleObjFitness};
use irongolem::golem::optimisers::genetic::parameters::generation_keeper::GenerationKeeper;
use irongolem::golem::optimisers::genetic::parameters::parameter::AdaptiveParameter;
use irongolem::golem::optimisers::genetic::parameters::population_size::{
    AdaptivePopulationSize, ConstRatePopulationSize,
};
use irongolem::golem::optimisers::genetic::parameters::sequence_iterator::SequenceIterator;
use irongolem::golem::optimisers::history::Individual;
use irongolem::golem::optimisers::objective::Objective;
use test_support::fixtures::{
    custom_objective_metrics, individual_with_primary_fitness, pop_size_sequence,
};

#[test]
fn test_const_pop_size_increases() {
    // def test_const_pop_size_increases():
    //     """ If there are too many fitness evaluation errors pop_size must increase to save population.
    //      With const pop_size population size mustn't be lower than initial pop_size. """
    //     initial_pop_size = 20
    //     pop_size = ConstRatePopulationSize(pop_size=initial_pop_size,
    //                                        offspring_rate=1)
    //
    //     # only one successfully evaluated individual
    //     population = [Individual(OptGraph(OptNode('rf')))]
    //     assert pop_size.next(population) >= initial_pop_size
    let initial_pop_size = 20;
    let pop_size = ConstRatePopulationSize::new(initial_pop_size, 1.0, None);
    let population = vec![individual_with_primary_fitness(0.0)];
    assert!(pop_size.next(&population) >= initial_pop_size);
}

#[test]
fn test_adaptive_pop_size_increases() {
    // def test_adaptive_pop_size_increases():
    //     """ If there are too many fitness evaluation errors pop_size must increase to save population.
    //      With adaptive pop_size population size must increase using iterator method `next`. """
    //     objective = Objective({'custom': custom_objective})
    //     generation_keeper = GenerationKeeper(objective=objective)
    //     pop_size = AdaptivePopulationSize(improvement_watcher=generation_keeper,
    //                                       progression_iterator=SequenceIterator(sequence_func=pop_size_sequence))
    //
    //     # only one successfully evaluated individual
    //     base_graph = OptGraph(OptNode('rf'))
    //     # to test only `too_many_fitness_eval_errors` case without `no_progress`
    //     fitness = [SingleObjFitness(primary_value=-0.8),
    //                SingleObjFitness(primary_value=-1)]
    //     population_0 = [Individual(base_graph, fitness=fitness[0])]
    //     generation_keeper.append(population_0)
    //     population_1 = [Individual(base_graph, fitness=fitness[1])]
    //     assert pop_size.next(population_1) >= len(population_1)
    let objective = Objective::new(custom_objective_metrics());
    let generation_keeper = Rc::new(RefCell::new(GenerationKeeper::new(Some(objective))));
    let pop_size = AdaptivePopulationSize::new(
        generation_keeper.clone(),
        SequenceIterator::new(pop_size_sequence, None, None, None),
        None,
    );
    let base_graph = Arc::new(GraphDelegate::new(LinkedGraphNode::from_name("rf")));
    let population_0 = vec![Individual::with_fitness(
        base_graph.clone(),
        Fitness::Single(SingleObjFitness::new(Some(-0.8), &[])),
    )];
    generation_keeper.borrow_mut().append(&population_0);
    let population_1 = vec![Individual::with_fitness(
        base_graph,
        Fitness::Single(SingleObjFitness::new(Some(-1.0), &[])),
    )];
    assert!(pop_size.next(&population_1) >= population_1.len());
}

#[test]
fn test_adaptive_max_pop_size() {
    // def test_adaptive_max_pop_size():
    //     """ Checks that `pop_size` never exceeds `max_pop_size`.
    //      In this test pop_size must be increased since there are too many evaluation errors
    //      (len(population added to generation keeper) == 1) and no progress in fitness. """
    //     objective = Objective({'custom': custom_objective})
    //     max_pop_size = 20
    //     generation_keeper = GenerationKeeper(objective=objective)
    //     pop_size = AdaptivePopulationSize(improvement_watcher=generation_keeper,
    //                                       progression_iterator=SequenceIterator(sequence_func=pop_size_sequence),
    //                                       max_pop_size=max_pop_size)
    //
    //     # only one successfully evaluated individual
    //     base_graph = OptGraph(OptNode('rf'))
    //     fitness = SingleObjFitness(primary_value=-0.8)
    //     population = [Individual(base_graph, fitness=fitness)]
    //     for i in range(10):
    //         generation_keeper.append(population)
    //         cur_pop_size = pop_size.next(population)
    //         print(cur_pop_size)
    //         assert cur_pop_size <= max_pop_size
    let objective = Objective::new(custom_objective_metrics());
    let max_pop_size = 20;
    let generation_keeper = Rc::new(RefCell::new(GenerationKeeper::new(Some(objective))));
    let pop_size = AdaptivePopulationSize::new(
        generation_keeper.clone(),
        SequenceIterator::new(pop_size_sequence, None, None, None),
        Some(max_pop_size),
    );
    let base_graph = Arc::new(GraphDelegate::new(LinkedGraphNode::from_name("rf")));
    let population = vec![Individual::with_fitness(
        base_graph,
        Fitness::Single(SingleObjFitness::new(Some(-0.8), &[])),
    )];
    for _ in 0..10 {
        generation_keeper.borrow_mut().append(&population);
        let cur_pop_size = pop_size.next(&population);
        assert!(cur_pop_size <= max_pop_size);
    }
}
