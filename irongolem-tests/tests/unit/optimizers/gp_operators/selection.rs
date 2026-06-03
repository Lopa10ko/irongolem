//! selection

#[test]
fn test_tournament_selection() {
    // def test_tournament_selection():
    //     num_of_inds = 3
    //     population = get_population()
    //     requirements = GPAlgorithmParameters(selection_types=[SelectionTypesEnum.tournament], pop_size=num_of_inds)
    //     selection = Selection(requirements)
    //     selected_individuals = selection(population)
    //     assert (all([ind in population for ind in selected_individuals]) and
    //             len(selected_individuals) == num_of_inds)
    assert!(false);
}

#[test]
fn test_random_selection() {
    // def test_random_selection():
    //     num_of_inds = 2
    //     population = get_population()
    //     selected_individuals = random_selection(population, pop_size=num_of_inds)
    //     assert (all([ind in population for ind in selected_individuals]) and
    //             len(selected_individuals) == num_of_inds)
    assert!(false);
}

#[test]
fn test_individuals_selection_random_individuals() {
    // def test_individuals_selection_random_individuals():
    //     num_of_inds = 2
    //     population = get_population()
    //     types = [SelectionTypesEnum.tournament]
    //     requirements = GPAlgorithmParameters(selection_types=types, pop_size=num_of_inds)
    //     selection = Selection(requirements)
    //     selected_individuals = selection(population)
    //     selected_individuals_ref = [str(ind) for ind in selected_individuals]
    //     assert (len(set(selected_individuals_ref)) == len(selected_individuals) and
    //             len(selected_individuals) == num_of_inds)
    assert!(false);
}

#[test]
fn test_individuals_selection_equality_individuals() {
    // def test_individuals_selection_equality_individuals():
    //     num_of_inds = 4
    //     one_ind = get_population()[0]
    //     types = [SelectionTypesEnum.tournament]
    //     requirements = GPAlgorithmParameters(selection_types=types, pop_size=num_of_inds)
    //     population = [one_ind for _ in range(4)]
    //     selection = Selection(requirements)
    //     selected_individuals = selection(population)
    //     selected_individuals_ref = [str(ind) for ind in selected_individuals]
    //     assert (len(selected_individuals) == num_of_inds and
    //             len(set(selected_individuals_ref)) == 1)
    assert!(false);
}

#[test]
fn test_custom_selection() {
    // def test_custom_selection():
    //     num_of_inds = 3
    //     population = get_population()
    //     requirements = GPAlgorithmParameters(selection_types=[custom_selection], pop_size=num_of_inds)
    //     selection = Selection(requirements)
    //     selected_individuals = selection(population)
    //     assert (all([ind in population for ind in selected_individuals]) and
    //             len(selected_individuals) == num_of_inds)
    assert!(false);
}
