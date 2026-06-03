//! generation_keeper

#[test]
fn test_archive_no_improvement() {
    // def test_archive_no_improvement():
    //     archive = generation_keeper(population1())
    //     assert archive.stagnation_iter_count == 0
    //     assert archive.is_any_improved
    //     assert archive.is_quality_improved and archive.is_complexity_improved
    //     assert archive.generation_num == 1
    //
    //     archive.append(population1())
    //     assert archive.stagnation_iter_count == 1
    //     assert not archive.is_any_improved
    //     assert not archive.is_quality_improved and not archive.is_complexity_improved
    //     assert archive.generation_num == 2
    assert!(false);
}

#[test]
fn test_archive_multiobj_one_improvement() {
    // def test_archive_multiobj_one_improvement():
    //     archive = generation_keeper(population1())
    //     previous_size = len(archive.best_individuals)
    //
    //     # second population has dominating individuals
    //     assert any(new_ind.fitness.dominates(population1()[1].fitness)
    //                for new_ind in population2())
    //     archive.append(population2())
    //
    //     assert archive.stagnation_iter_count == 0
    //     assert archive.is_any_improved
    //     assert archive.generation_num == 2
    //     # plus one non-dominated individual
    //     # minus one strongly dominated individual (substituted by better one)
    //     assert len(archive.best_individuals) == previous_size + 1
    //     assert archive.is_complexity_improved
    //     assert not archive.is_quality_improved
    assert!(false);
}
