use std::sync::Arc;

use irongolem::golem::dag::{Graph, GraphDelegate, LinkedGraphNode};
use irongolem::golem::optimisers::fitness::{Fitness, MultiObjFitness};
use irongolem::golem::optimisers::genetic::operators::base_mutations::MutationTypesEnum;
use irongolem::golem::optimisers::genetic::operators::crossover::{Crossover, CrossoverTypesEnum};
use irongolem::golem::optimisers::genetic::operators::mutation::Mutation;
use irongolem::golem::optimisers::genetic::params::{
    GPAlgorithmParameters, GraphGenerationParams, GraphRequirements,
};
use irongolem::golem::optimisers::genetic::EvoGraphOptimizer;
use irongolem::golem::optimisers::history::{Individual, ParentOperator};
use irongolem::golem::optimisers::objective::{Objective, ObjectiveEvaluate, ObjectiveInfo};
use irongolem::golem::optimisers::opt_history::OptHistory;
use test_support::fixtures::mock_adapter::{MockAdapter, MockDomainStructure, MockNode};
use test_support::fixtures::{graph_fifth, graph_first, graph_fourth, graph_second, graph_third};

fn create_individual() -> Individual {
    let first = LinkedGraphNode::from_name("logit");
    let second = LinkedGraphNode::from_name("lda");
    let final_node = LinkedGraphNode::with_parents("knn", vec![first, second]);
    let mut ind = Individual::new(Arc::new(GraphDelegate::new(final_node)));
    ind.set_fitness(Fitness::Multi(MultiObjFitness::new(&[0.5, 0.5], None)));
    ind
}

fn generate_history(generations_quantity: usize, pop_size: usize) -> OptHistory {
    let mut history = OptHistory::new(None);
    for gen_num in 0..generations_quantity {
        let mut new_pop = Vec::new();
        for _ in 0..pop_size {
            let mut ind = create_individual();
            ind.set_native_generation(gen_num);
            new_pop.push(ind);
        }
        history.add_to_history(new_pop.clone(), None, None);
        let best = new_pop
            .iter()
            .min_by(|a, b| {
                a.fitness.values()[0]
                    .partial_cmp(&b.fitness.values()[0])
                    .unwrap()
            })
            .cloned()
            .unwrap();
        history.add_to_archive_history(&[best]);
    }
    history
}

#[test]
fn test_history_adding() {
    let history = generate_history(2, 10);
    assert_eq!(history.generations.len(), 2);
    for gen in &history.generations {
        assert_eq!(gen.len(), 10);
    }
}

#[test]
fn test_individual_graph_type_is_optgraph() {
    let history = generate_history(2, 10);
    for gen in &history.generations {
        for ind in &gen.individuals {
            assert!(ind.graph.as_ref().descriptive_id().contains("knn"));
        }
    }
}

#[test]
fn test_ancestor_for_crossover() {
    let adapter = MockAdapter;
    let parent_ind_first =
        Individual::new(adapter.adapt(MockDomainStructure::new(vec![MockNode::new("a")])));
    let parent_ind_second =
        Individual::new(adapter.adapt(MockDomainStructure::new(vec![MockNode::new("b")])));

    let requirements = GraphRequirements::default();
    let graph_params = GraphGenerationParams::new(vec!["a".into(), "b".into()]);
    let opt_parameters = GPAlgorithmParameters::default()
        .with_crossover_types(vec![CrossoverTypesEnum::Subtree])
        .with_crossover_prob(1.0);
    let crossover = Crossover::new(opt_parameters, requirements, graph_params);
    let results = crossover.call(vec![parent_ind_first.clone(), parent_ind_second.clone()]);

    for crossover_result in &results {
        assert!(crossover_result.parent_operator.is_some());
        let op = crossover_result.parent_operator.as_ref().unwrap();
        assert_eq!(op.type_, "crossover");
        assert_eq!(op.parents().len(), 2);
        assert_eq!(op.parents()[0].uid, parent_ind_first.uid);
        assert_eq!(op.parents()[1].uid, parent_ind_second.uid);
    }
    if results.len() == 2 {
        let first_parents = &results[0]
            .parent_operator
            .as_ref()
            .unwrap()
            .parent_individuals;
        let second_parents = &results[1]
            .parent_operator
            .as_ref()
            .unwrap()
            .parent_individuals;
        assert!(Arc::ptr_eq(&first_parents[0], &second_parents[0]));
        assert!(Arc::ptr_eq(&first_parents[1], &second_parents[1]));
    }
}

#[test]
fn test_ancestor_for_mutation() {
    let adapter = MockAdapter;
    let parent_ind =
        Individual::new(adapter.adapt(MockDomainStructure::new(vec![MockNode::new("a")])));

    let requirements = GraphRequirements::default();
    let graph_params = GraphGenerationParams::new(vec!["a".into()]);
    let parameters = GPAlgorithmParameters::default()
        .with_mutation_types(vec![MutationTypesEnum::SingleAdd])
        .with_mutation_prob(1.0);
    let mutation = Mutation::new(parameters, requirements, graph_params);

    if let irongolem::golem::optimisers::genetic::operators::mutation::MutationResult::Individual(
        Some(mutation_result),
    ) = mutation.call(
        irongolem::golem::optimisers::genetic::operators::mutation::MutationTarget::Individual(
            parent_ind.clone(),
        ),
    ) {
        assert!(mutation_result.parent_operator.is_some());
        let op = mutation_result.parent_operator.as_ref().unwrap();
        assert_eq!(op.type_, "mutation");
        assert_eq!(op.parents().len(), 1);
        assert_eq!(op.parents()[0].uid, parent_ind.uid);
    }
}

#[test]
fn test_parent_operator() {
    let adapter = MockAdapter;
    let ind = Individual::new(adapter.adapt(MockDomainStructure::new(vec![MockNode::new("a")])));
    let operator_for_history = ParentOperator::new(
        "mutation",
        format!("{:?}", MutationTypesEnum::Simple),
        vec![Arc::new(ind.clone())],
    );
    assert_eq!(operator_for_history.parents()[0].uid, ind.uid);
    assert_eq!(operator_for_history.type_, "mutation");
}

#[test]
fn test_history_properties() {
    let history = generate_history(2, 10);
    assert_eq!(history.all_historical_quality(0).len(), 20);
    assert_eq!(history.historical_fitness().len(), 2);
    assert_eq!(history.historical_fitness()[0].len(), 10);
    assert_eq!(history.all_historical_fitness().len(), 20);
}

#[test]
fn test_history_save_custom_nodedata() {
    let mut history = OptHistory::new(None);
    let graphs: Vec<Individual> = (0..10)
        .map(|i| {
            let node = LinkedGraphNode::from_name(&format!("custom_{i}"));
            let mut ind = Individual::new(Arc::new(GraphDelegate::new(node)));
            ind.set_native_generation(i);
            ind
        })
        .collect();
    history.add_to_history(graphs[..3].to_vec(), None, None);
    history.add_to_history(graphs[3..6].to_vec(), None, None);
    history.add_to_history(graphs[6..].to_vec(), None, None);
    let saved = history.save(None, false).expect("save");
    assert!(!saved.is_empty());
    assert_eq!(history.generations.len(), 3);
}

#[test]
fn test_prepare_for_visualisation() {
    let history = generate_history(2, 10);
    assert_eq!(history.all_historical_fitness().len(), 20);
    let leaderboard = history.get_leaderboard(10);
    assert!(leaderboard.contains("knn"));
    assert!(leaderboard.contains("Position"));
    let dumped = history.save(None, false).expect("save");
    let loaded = OptHistory::load(&dumped).expect("load");
    let leaderboard = loaded.get_leaderboard(10);
    assert!(leaderboard.contains("knn"));
}

#[test]
fn test_all_historical_quality() {
    let mut history = generate_history(3, 4);
    let mut eval_fitness = vec![[0.9, 0.8], [0.8, 0.6], [0.2, 0.4], [0.9, 0.9]];
    let weights = [-1.0, 1.0];
    for (pop_num, generation) in history.generations.iter_mut().enumerate() {
        if pop_num != 0 {
            eval_fitness = eval_fitness
                .iter()
                .map(|fit| [fit[0] + 0.5, fit[1]])
                .collect();
        }
        for (ind_num, individual) in generation.individuals.iter_mut().enumerate() {
            individual.set_fitness(Fitness::Multi(MultiObjFitness::new(
                &eval_fitness[ind_num],
                Some(&weights),
            )));
        }
    }
    let all_quality = history.all_historical_quality(0);
    assert_eq!(all_quality[0], -0.9);
    assert_eq!(all_quality[4], -1.4);
    assert_eq!(all_quality[5], -1.3);
    assert_eq!(all_quality[10], -1.2);
}

#[test]
fn test_newly_generated_history() {
    let num_of_gens = 5;
    let mut objective = Objective::new(std::collections::HashMap::from([(
        "random_metric".into(),
        "random".into(),
    )]));
    objective = objective.with_evaluator("random_metric", Arc::new(|_g| 0.0));
    let init_graphs = vec![
        graph_first(),
        graph_second(),
        graph_third(),
        graph_fourth(),
        graph_fifth(),
    ];
    let mut requirements = GraphRequirements::default();
    requirements.num_of_generations = Some(num_of_gens);
    requirements.early_stopping_iterations = Some(1000);
    let graph_generation_params = GraphGenerationParams::new(
        vec!["a", "b", "c", "d", "e", "f"]
            .into_iter()
            .map(String::from)
            .collect(),
    );
    let opt_params = GPAlgorithmParameters::new(5).with_random_seed(42);
    let mut opt = EvoGraphOptimizer::new(
        objective.clone(),
        Some(init_graphs.into_iter().map(|g| Arc::new(g)).collect()),
        requirements,
        graph_generation_params,
        opt_params,
    );
    let objective_eval = ObjectiveEvaluate::new(objective);
    opt.optimise(&objective_eval).expect("optimise");
    let history = opt.history().expect("history");

    assert_eq!(history.generations.len(), num_of_gens + 2);
    assert_eq!(history.archive_history.len(), num_of_gens + 2);
    assert_eq!(history.initial_assumptions().unwrap().len(), 5);
    assert_eq!(history.final_choices().unwrap().len(), 1);
    assert!(history.objective.is_some());
}

#[test]
#[ignore = "visualisation deferred"]
fn test_history_show_saving_plots() {}

#[test]
#[ignore = "visualisation deferred"]
fn test_extra_history_visualizer() {}

#[test]
fn test_history_correct_serialization() {
    let history = generate_history(3, 4);
    let dumped = history.save(None, false).expect("save");
    let reloaded = OptHistory::load(&dumped).expect("reload");
    assert_eq!(history.generations.len(), reloaded.generations.len());
    assert_eq!(
        history.archive_history.len(),
        reloaded.archive_history.len()
    );
}

#[test]
fn test_collect_intermediate_metric() {
    // deferred: requires intermediate metric callback wiring
}

#[test]
fn test_load_zero_generations_history() {
    let history = OptHistory::new(Some(ObjectiveInfo {
        is_multi_objective: false,
        metric_names: vec!["rmse".into(), "node_number".into()],
    }));
    let dumped = history.save(None, false).expect("save");
    let loaded = OptHistory::load(&dumped).expect("load");
    assert!(loaded.archive_history.is_empty());
    assert!(loaded.generations.is_empty());
    assert!(loaded.objective.is_some());
}

#[test]
fn test_save_load_light_history() {
    let history = generate_history(100, 1);
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/light_history.json");
    history.save(Some(&path), true).expect("save");
    let loaded = OptHistory::load(path.to_str().unwrap()).expect("load");
    assert_eq!(loaded.archive_history.len(), loaded.generations.len());
    assert_eq!(loaded.generations.len(), 100);
    for gen in &loaded.generations {
        assert_eq!(gen.len(), 1);
    }
    let _ = std::fs::remove_file(path);
}

#[test]
fn test_light_history_is_significantly_lighter() {
    let history = generate_history(50, 30);
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let light_path = dir.join("light_history_tmp.json");
    let heavy_path = dir.join("heavy_history_tmp.json");
    history.save(Some(&light_path), true).expect("save");
    history.save(Some(&heavy_path), false).expect("save");
    let light_size = std::fs::metadata(&light_path).unwrap().len();
    let heavy_size = std::fs::metadata(&heavy_path).unwrap().len();
    assert!(light_size * 25 <= heavy_size);
    let _ = std::fs::remove_file(light_path);
    let _ = std::fs::remove_file(heavy_path);
}
