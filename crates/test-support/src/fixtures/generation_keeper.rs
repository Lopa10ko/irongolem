use std::sync::Arc;

use irongolem::golem::dag::{GraphDelegate, LinkedGraphNode};
use irongolem::golem::optimisers::fitness::{Fitness, MultiObjFitness};
use irongolem::golem::optimisers::genetic::operators::PopulationT;
use irongolem::golem::optimisers::genetic::parameters::generation_keeper::GenerationKeeper;
use irongolem::golem::optimisers::history::Individual;
use irongolem::golem::optimisers::objective::Objective;

fn create_individual(fitness: Fitness) -> Individual {
    let graph = Arc::new(GraphDelegate::new(LinkedGraphNode::from_name("n1")));
    Individual::with_fitness(graph, fitness)
}

pub fn population1() -> PopulationT {
    vec![
        create_individual(Fitness::Multi(MultiObjFitness::new(
            &[2.0, 4.0],
            Some(&[-1.0, -1.0]),
        ))),
        create_individual(Fitness::Multi(MultiObjFitness::new(
            &[3.0, 2.0],
            Some(&[-1.0, -1.0]),
        ))),
    ]
}

pub fn population2() -> PopulationT {
    vec![
        create_individual(Fitness::Multi(MultiObjFitness::new(
            &[1.0, 5.0],
            Some(&[-1.0, -1.0]),
        ))),
        create_individual(Fitness::Multi(MultiObjFitness::new(
            &[3.0, 3.0],
            Some(&[-1.0, -1.0]),
        ))),
    ]
}

pub fn generation_keeper(init_population: PopulationT) -> GenerationKeeper {
    let mut quality = std::collections::HashMap::new();
    quality.insert("random_metric".into(), "random".into());
    let mut complexity = std::collections::HashMap::new();
    complexity.insert("depth".into(), "depth".into());
    let objective = Objective::multi_objective(quality, complexity);
    GenerationKeeper::with_keep_n_best(Some(objective), 1).with_initial_generation(init_population)
}
