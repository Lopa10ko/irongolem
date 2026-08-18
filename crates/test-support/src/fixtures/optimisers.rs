use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use irongolem::golem::adapter::DirectAdapter;
use irongolem::golem::dag::{GraphDelegate, LinkedGraphNode};
use irongolem::golem::optimisers::fitness::{Fitness, SingleObjFitness};
use irongolem::golem::optimisers::genetic::operators::crossover::Crossover;
use irongolem::golem::optimisers::genetic::operators::crossover::CrossoverTypesEnum;
use irongolem::golem::optimisers::genetic::operators::mutation::Mutation;
use irongolem::golem::optimisers::genetic::operators::reproduction::ReproductionController;
use irongolem::golem::optimisers::genetic::operators::selection::Selection;
use irongolem::golem::optimisers::genetic::operators::EvaluationOperator;
use irongolem::golem::optimisers::genetic::operators::MutationTypesEnum;
use irongolem::golem::optimisers::genetic::operators::PopulationT;
use irongolem::golem::optimisers::genetic::params::{
    GPAlgorithmParameters, GraphGenerationParams, GraphRequirements, SelectionType,
};
use irongolem::golem::optimisers::genetic::rng::{random_choice, sample, set_random_seed};
use irongolem::golem::optimisers::history::Individual;
use irongolem::golem::optimisers::objective::Objective;

use super::graphs::{
    graph_fifth, graph_first, graph_fourth, graph_second, graph_third, simple_linear_graph,
    tree_graph,
};
use super::metrics::RandomMetric;

pub struct MutationParams {
    pub requirements: GraphRequirements,
    pub graph_gen_params: GraphGenerationParams,
    pub parameters: GPAlgorithmParameters,
}

pub fn get_objective(graph: Arc<GraphDelegate>) -> Fitness {
    let mut metrics = HashMap::new();
    metrics.insert("random_metric".into(), "random".into());
    let objective = Objective::new(metrics);
    let _ = RandomMetric::get_value(graph.clone(), Duration::ZERO);
    objective.evaluate(graph)
}

pub fn get_population() -> PopulationT {
    let adapter = DirectAdapter;
    let graphs = [
        graph_first(),
        graph_second(),
        graph_third(),
        graph_fourth(),
        graph_fifth(),
    ];
    let mut population: PopulationT = graphs
        .into_iter()
        .map(|g| Individual::new(adapter.adapt(g)))
        .collect();
    for ind in &mut population {
        ind.set_fitness(get_objective(ind.graph.clone()));
    }
    population
}

pub fn get_mutation_params(
    mutation_types: Option<Vec<MutationTypesEnum>>,
    requirements: Option<GraphRequirements>,
    mutation_prob: f64,
) -> MutationParams {
    let requirements = requirements.unwrap_or_default();
    let graph_generation_params = GraphGenerationParams::new(
        vec!["a", "b", "c", "d", "e", "f"]
            .into_iter()
            .map(String::from)
            .collect(),
    );
    let mutation_types = mutation_types.unwrap_or_else(MutationTypesEnum::rich_mutation_set);
    let parameters = GPAlgorithmParameters::default()
        .with_mutation_types(mutation_types)
        .with_mutation_prob(mutation_prob);
    MutationParams {
        requirements,
        graph_gen_params: graph_generation_params,
        parameters,
    }
}

pub fn get_rand_population(pop_size: usize) -> PopulationT {
    let adapter = DirectAdapter;
    let templates = [
        graph_first(),
        graph_second(),
        graph_third(),
        graph_fourth(),
        graph_fifth(),
        tree_graph(),
        simple_linear_graph(),
    ];
    (0..pop_size)
        .filter_map(|_| {
            let template = random_choice(&templates)?;
            Some(Individual::new(adapter.adapt(template.deep_clone())))
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct MockEvaluator {
    pub success_prob: f64,
}

impl MockEvaluator {
    pub fn new(success_prob: f64) -> Self {
        Self { success_prob }
    }

    pub fn as_operator(self) -> EvaluationOperator {
        Arc::new(move |pop| {
            let n_valid = (self.success_prob * pop.len() as f64).ceil() as usize;
            let n_valid = n_valid.min(pop.len());
            irongolem::golem::optimisers::genetic::rng::sample(&pop, n_valid)
        })
    }
}

pub fn mock_evaluator(success_prob: f64) -> EvaluationOperator {
    MockEvaluator::new(success_prob).as_operator()
}

pub fn individual_with_primary_fitness(primary: f64) -> Individual {
    Individual::with_fitness(
        Arc::new(GraphDelegate::new(LinkedGraphNode::from_name("rf"))),
        Fitness::Single(SingleObjFitness::new(Some(primary), &[])),
    )
}

pub fn get_graph_with_operation(operation: &str) -> GraphDelegate {
    let node1 = LinkedGraphNode::from_name(operation);
    let node2 = LinkedGraphNode::from_name(operation);
    let node3 = LinkedGraphNode::with_parents(operation, vec![node1, node2]);
    GraphDelegate::new(node3)
}

pub fn population_with_structural_duplicates(operations: &[&str]) -> PopulationT {
    let adapter = DirectAdapter;
    let mut population = Vec::new();
    for op in operations {
        let ind = Individual::new(adapter.adapt(get_graph_with_operation(op)));
        population.push(ind.clone());
        population.push(ind);
    }
    population
}

pub fn custom_selection_fn() -> SelectionType {
    SelectionType::Custom(Arc::new(|population, pop_size| {
        sample(population, pop_size)
    }))
}

pub fn identity_evaluator() -> EvaluationOperator {
    Arc::new(|pop| pop)
}

pub fn pop_size_sequence(n: usize) -> usize {
    let mut a = 2usize;
    let mut b = 3usize;
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a.max(3)
}

pub fn custom_objective_metrics() -> HashMap<String, String> {
    HashMap::from([("custom".into(), "custom".into())])
}

pub fn reproducer_fixture() -> ReproductionController {
    reproducer_with_pop_size(30)
}

pub fn reproducer_with_pop_size(pop_size: usize) -> ReproductionController {
    set_random_seed(42);
    let requirements = GraphRequirements::default();
    let mut graph_gen_params = GraphGenerationParams::new(vec!["x".into()]);
    graph_gen_params.verifier = Arc::new(|_| true);
    let mut params = GPAlgorithmParameters::new(pop_size).with_random_seed(42);
    params.max_pop_size = Some(100);
    params.offspring_rate = 0.2;
    params.required_valid_ratio = 0.9;
    params.mutation_prob = 1.0;
    params.crossover_types = vec![CrossoverTypesEnum::None];
    params.mutation_types = vec![MutationTypesEnum::SingleAdd, MutationTypesEnum::SingleDrop];

    let mutation = Mutation::new(
        params.clone(),
        requirements.clone(),
        graph_gen_params.clone(),
    );
    let crossover = Crossover::new(params.clone(), requirements.clone(), graph_gen_params);
    let selection = Selection::new(params.clone(), requirements);
    ReproductionController::new(params, selection, mutation, crossover)
}

pub fn elitism_set_up() -> (PopulationT, PopulationT) {
    let adapter = DirectAdapter;
    let graphs = [
        graph_first(),
        graph_second(),
        graph_third(),
        graph_fourth(),
        graph_fifth(),
    ];
    let mut population: PopulationT = graphs
        .into_iter()
        .map(|g| Individual::new(adapter.adapt(g)))
        .collect();
    for ind in &mut population {
        ind.set_fitness(get_objective(ind.graph.clone()));
    }
    let best_individuals = population[2..].to_vec();
    let population = population[..4].to_vec();
    (best_individuals, population)
}

pub fn is_close(left: f64, right: f64, rtol: f64) -> bool {
    (left - right).abs() <= 1e-8 + rtol * right.abs()
}
