//! selection

use irongolem::golem::optimisers::genetic::operators::selection::{random_selection, Selection};
use irongolem::golem::optimisers::genetic::params::{
    GPAlgorithmParameters, GraphRequirements, SelectionType,
};
use irongolem::golem::optimisers::genetic::rng::GeneticRng;
use test_support::fixtures::{custom_selection_fn, get_population};

fn population_contains(
    population: &[irongolem::golem::optimisers::history::Individual],
    ind: &irongolem::golem::optimisers::history::Individual,
) -> bool {
    population.iter().any(|p| p.uid == ind.uid)
}

#[test]
fn test_tournament_selection() {
    let num_of_inds = 3;
    let population = get_population();
    let parameters = GPAlgorithmParameters::new(num_of_inds)
        .with_selection_types(vec![SelectionType::Tournament]);
    let selection = Selection::new(parameters, GraphRequirements::default());
    let selected_individuals = selection.call(population.clone(), None);
    assert!(selected_individuals
        .iter()
        .all(|ind| population_contains(&population, ind)));
    assert_eq!(selected_individuals.len(), num_of_inds);
}

#[test]
fn test_random_selection() {
    let num_of_inds = 2;
    let population = get_population();
    let selected_individuals =
        random_selection(population.clone(), num_of_inds, &GeneticRng::entropy());
    assert!(selected_individuals
        .iter()
        .all(|ind| population_contains(&population, ind)));
    assert_eq!(selected_individuals.len(), num_of_inds);
}

#[test]
fn test_individuals_selection_random_individuals() {
    let num_of_inds = 2;
    let population = get_population();
    let parameters = GPAlgorithmParameters::new(num_of_inds)
        .with_selection_types(vec![SelectionType::Tournament]);
    let selection = Selection::new(parameters, GraphRequirements::default());
    let selected_individuals = selection.call(population, None);
    let selected_refs: std::collections::HashSet<_> = selected_individuals
        .iter()
        .map(|ind| ind.uid.clone())
        .collect();
    assert_eq!(selected_refs.len(), selected_individuals.len());
    assert_eq!(selected_individuals.len(), num_of_inds);
}

#[test]
fn test_individuals_selection_equality_individuals() {
    let num_of_inds = 4;
    let one_ind = get_population().swap_remove(0);
    let parameters = GPAlgorithmParameters::new(num_of_inds)
        .with_selection_types(vec![SelectionType::Tournament]);
    let population = vec![one_ind.clone(); 4];
    let selection = Selection::new(parameters, GraphRequirements::default());
    let selected_individuals = selection.call(population, None);
    let selected_refs: std::collections::HashSet<_> = selected_individuals
        .iter()
        .map(|ind| ind.uid.clone())
        .collect();
    assert_eq!(selected_individuals.len(), num_of_inds);
    assert_eq!(selected_refs.len(), 1);
}

#[test]
fn test_custom_selection() {
    let num_of_inds = 3;
    let population = get_population();
    let parameters =
        GPAlgorithmParameters::new(num_of_inds).with_selection_types(vec![custom_selection_fn()]);
    let selection = Selection::new(parameters, GraphRequirements::default());
    let selected_individuals = selection.call(population.clone(), None);
    assert!(selected_individuals
        .iter()
        .all(|ind| population_contains(&population, ind)));
    assert_eq!(selected_individuals.len(), num_of_inds);
}

#[test]
fn test_spea2_returns_pop_size() {
    use irongolem::golem::optimisers::fitness::{Fitness, MultiObjFitness};
    use irongolem::golem::optimisers::genetic::operators::selection::spea2_selection;

    let mut population = get_population();
    let len = population.len();
    for (i, ind) in population.iter_mut().enumerate() {
        ind.set_fitness(Fitness::Multi(MultiObjFitness::new(
            &[i as f64, (len - i) as f64],
            None,
        )));
    }
    let selected = spea2_selection(population, 3, &GeneticRng::seeded(42));
    assert_eq!(selected.len(), 3);
}

#[test]
fn test_spea2_prefers_non_dominated() {
    use irongolem::golem::dag::{GraphDelegate, LinkedGraphNode};
    use irongolem::golem::optimisers::fitness::{Fitness, MultiObjFitness};
    use irongolem::golem::optimisers::genetic::operators::selection::spea2_selection;
    use irongolem::golem::optimisers::history::Individual;
    use std::sync::Arc;

    let dominated = Individual::with_fitness(
        Arc::new(GraphDelegate::new(LinkedGraphNode::from_name("d"))),
        Fitness::Multi(MultiObjFitness::new(&[1.0, 1.0], None)),
    );
    let non_dominated_a = Individual::with_fitness(
        Arc::new(GraphDelegate::new(LinkedGraphNode::from_name("a"))),
        Fitness::Multi(MultiObjFitness::new(&[0.0, 1.0], None)),
    );
    let non_dominated_b = Individual::with_fitness(
        Arc::new(GraphDelegate::new(LinkedGraphNode::from_name("b"))),
        Fitness::Multi(MultiObjFitness::new(&[1.0, 0.0], None)),
    );
    let filler = Individual::with_fitness(
        Arc::new(GraphDelegate::new(LinkedGraphNode::from_name("c"))),
        Fitness::Multi(MultiObjFitness::new(&[2.0, 2.0], None)),
    );

    let population = vec![
        dominated.clone(),
        non_dominated_a.clone(),
        non_dominated_b.clone(),
        filler,
    ];
    let selected = spea2_selection(population, 2, &GeneticRng::seeded(7));
    let uids: std::collections::HashSet<_> = selected.iter().map(|i| i.uid.clone()).collect();
    assert!(uids.contains(&non_dominated_a.uid));
    assert!(uids.contains(&non_dominated_b.uid));
    assert!(!uids.contains(&dominated.uid));
}

#[test]
fn test_spea2_archive_trimming() {
    use irongolem::golem::dag::{GraphDelegate, LinkedGraphNode};
    use irongolem::golem::optimisers::fitness::{Fitness, MultiObjFitness};
    use irongolem::golem::optimisers::genetic::operators::selection::spea2_selection;
    use irongolem::golem::optimisers::history::Individual;
    use std::sync::Arc;

    let make = |name: &str, f0: f64, f1: f64| {
        Individual::with_fitness(
            Arc::new(GraphDelegate::new(LinkedGraphNode::from_name(name))),
            Fitness::Multi(MultiObjFitness::new(&[f0, f1], None)),
        )
    };

    let population = vec![
        make("a", 0.0, 1.0),
        make("b", 1.0, 0.0),
        make("c", 0.5, 0.5),
        make("d", 0.4, 0.6),
        make("e", 0.6, 0.4),
        make("f", 0.3, 0.7),
    ];
    let selected_a = spea2_selection(population.clone(), 3, &GeneticRng::seeded(99));
    let selected_b = spea2_selection(population, 3, &GeneticRng::seeded(99));
    assert_eq!(selected_a.len(), 3);
    let uids_a: std::collections::HashSet<_> = selected_a.iter().map(|i| i.uid.clone()).collect();
    let uids_b: std::collections::HashSet<_> = selected_b.iter().map(|i| i.uid.clone()).collect();
    assert_eq!(uids_a, uids_b);
}
