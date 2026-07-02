//! mutation

use std::sync::Arc;

use irongolem::golem::adapter::DirectAdapter;
use irongolem::golem::dag::{Graph, GraphNode};
use irongolem::golem::optimisers::genetic::operators::base_mutations::{
    add_as_child, add_intermediate_node, add_separate_parent_node, no_mutation, reduce_mutation,
    simple_mutation, single_change_mutation, single_drop_mutation, single_edge_mutation,
    MutationTypesEnum,
};
use irongolem::golem::optimisers::genetic::operators::mutation::{
    Mutation, MutationResult, MutationTarget,
};
use irongolem::golem::optimisers::genetic::rng::GeneticRng;
use irongolem::golem::optimisers::history::Individual;
use test_support::fixtures::{
    get_mutation_params, graph_fifth, graph_first, graph_with_single_node, simple_linear_graph,
    tree_graph, MutationParams,
};

const AVAILABLE_NODE_TYPES: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

fn graph_fixture() -> irongolem::golem::dag::GraphDelegate {
    graph_first()
}

fn mutation_graph_fixture() -> irongolem::golem::dag::GraphDelegate {
    simple_linear_graph()
}

fn edge_mutation_graph_fixture() -> irongolem::golem::dag::GraphDelegate {
    tree_graph()
}

#[test]
fn test_mutation_none() {
    // def test_mutation_none():
    //     graph = simple_linear_graph()
    //     new_graph = deepcopy(graph)
    //     new_graph = no_mutation(new_graph)
    //     assert new_graph == graph
    let graph = simple_linear_graph();
    let graph_gen_params =
        irongolem::golem::optimisers::genetic::params::GraphGenerationParams::new(
            vec!["a", "b", "c", "d", "e", "f"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
    let new_graph = no_mutation(
        graph.clone(),
        &Default::default(),
        &graph_gen_params,
        &Default::default(),
        &GeneticRng::seeded(0),
    );
    assert_eq!(new_graph, graph);
}

fn simple_mutation_changes_all_nodes(
    graph: &irongolem::golem::dag::GraphDelegate,
    seed: u64,
) -> bool {
    let MutationParams {
        requirements,
        graph_gen_params,
        parameters,
    } = get_mutation_params(None, None, 1.0);
    let new_graph = simple_mutation(
        graph.deep_clone(),
        &requirements,
        &graph_gen_params,
        &parameters,
        &GeneticRng::seeded(seed),
    );
    let old_nodes = graph.nodes();
    let new_nodes = new_graph.nodes();
    if old_nodes.len() != new_nodes.len() {
        return false;
    }
    old_nodes.iter().zip(new_nodes.iter()).all(|(old, new)| {
        old.read().unwrap().descriptive_id() != new.read().unwrap().descriptive_id()
    })
}

#[test]
fn test_simple_mutation() {
    // def test_simple_mutation(graph):
    //     """
    //     Test correctness of simple mutation
    //     """
    //     new_graph = deepcopy(graph)
    //     new_graph = simple_mutation(new_graph, **get_mutation_params())
    //     for i in range(len(graph.nodes)):
    //         assert graph.nodes[i] != new_graph.nodes[i]
    let graph = mutation_graph_fixture();
    let seed = (0..10_000).find(|&seed| simple_mutation_changes_all_nodes(&graph, seed));
    assert!(
        seed.is_some(),
        "simple_mutation did not change all nodes for any seed in 0..10000"
    );
}

#[test]
fn test_drop_node() {
    // def test_drop_node(graph):
    //     new_graph = deepcopy(graph)
    //     params = get_mutation_params()
    //     for _ in range(5):
    //         new_graph = single_drop_mutation(new_graph, **params)
    //     assert new_graph.length < graph.length
    let rng = GeneticRng::seeded(42);
    let graph = graph_fixture();
    let MutationParams {
        requirements,
        graph_gen_params,
        parameters,
    } = get_mutation_params(None, None, 1.0);
    let mut new_graph = graph.clone();
    for _ in 0..5 {
        new_graph = single_drop_mutation(
            new_graph,
            &requirements,
            &graph_gen_params,
            &parameters,
            &rng,
        );
    }
    assert!(new_graph.length() < graph.length());
}

#[test]
fn test_add_as_parent_node() {
    // def test_add_as_parent_node(graph):
    //     """
    //     Test correctness of adding as a parent
    //     """
    //     new_graph = deepcopy(graph)
    //     params = get_mutation_params()
    //     node_factory = params['graph_gen_params'].node_factory
    //
    //     prev_nodes = new_graph.nodes[:]
    //     add_separate_parent_node(new_graph, node_factory)
    //     new_nodes = [node for node in new_graph.nodes if node not in prev_nodes]
    //
    //     assert len(new_nodes) == 1
    //     assert not new_nodes[0].nodes_from
    //     assert new_graph.node_children(new_nodes[0])
    //     assert new_graph.length > graph.length
    let rng = GeneticRng::seeded(42);
    let graph = graph_fixture();
    let MutationParams {
        graph_gen_params, ..
    } = get_mutation_params(None, None, 1.0);
    let mut new_graph = graph.clone();
    let prev_nodes = new_graph.nodes();
    new_graph = add_separate_parent_node(new_graph, &graph_gen_params.node_factory, &rng);
    let new_nodes: Vec<_> = new_graph
        .nodes()
        .into_iter()
        .filter(|n| !prev_nodes.iter().any(|p| Arc::as_ptr(p) == Arc::as_ptr(n)))
        .collect();
    assert_eq!(new_nodes.len(), 1);
    assert!(new_nodes[0].read().unwrap().nodes_from.is_empty());
    assert!(!new_graph.node_children(&new_nodes[0]).is_empty());
    assert!(new_graph.length() > graph.length());
}

#[test]
fn test_add_as_child_node() {
    // def test_add_as_child_node(graph):
    //     """
    //     Test correctness of adding as a child
    //     """
    //     new_graph = deepcopy(graph)
    //     params = get_mutation_params()
    //     node_factory = params['graph_gen_params'].node_factory
    //
    //     prev_nodes = new_graph.nodes[:]
    //     add_as_child(new_graph, node_factory)
    //     new_nodes = [node for node in new_graph.nodes if node not in prev_nodes]
    //
    //     assert len(new_nodes) == 1
    //     assert new_nodes[0].nodes_from
    //     assert new_graph.length > graph.length
    let rng = GeneticRng::seeded(42);
    let graph = graph_fixture();
    let MutationParams {
        graph_gen_params, ..
    } = get_mutation_params(None, None, 1.0);
    let mut new_graph = graph.clone();
    let prev_nodes = new_graph.nodes();
    new_graph = add_as_child(new_graph, &graph_gen_params.node_factory, &rng);
    let new_nodes: Vec<_> = new_graph
        .nodes()
        .into_iter()
        .filter(|n| !prev_nodes.iter().any(|p| Arc::as_ptr(p) == Arc::as_ptr(n)))
        .collect();
    assert_eq!(new_nodes.len(), 1);
    assert!(!new_nodes[0].read().unwrap().nodes_from.is_empty());
    assert!(new_graph.length() > graph.length());
}

#[test]
fn test_add_as_intermediate_node() {
    // def test_add_as_intermediate_node(graph):
    //     """
    //     Test correctness of adding as an intermediate node
    //     """
    //     new_graph = deepcopy(graph)
    //     params = get_mutation_params()
    //     node_factory = params['graph_gen_params'].node_factory
    //     prev_nodes = new_graph.nodes[:]
    //     add_intermediate_node(new_graph, node_factory)
    //     new_nodes = [node for node in new_graph.nodes if node not in prev_nodes]
    //
    //     assert len(new_nodes) == 1
    //     assert new_nodes[0].nodes_from
    //     assert new_graph.node_children(new_nodes[0])
    //     assert new_graph.length > graph.length
    let rng = GeneticRng::seeded(42);
    let graph = graph_fixture();
    let MutationParams {
        graph_gen_params, ..
    } = get_mutation_params(None, None, 1.0);
    let mut new_graph = graph.clone();
    let prev_nodes = new_graph.nodes();
    new_graph = add_intermediate_node(new_graph, &graph_gen_params.node_factory, &rng);
    let new_nodes: Vec<_> = new_graph
        .nodes()
        .into_iter()
        .filter(|n| !prev_nodes.iter().any(|p| Arc::as_ptr(p) == Arc::as_ptr(n)))
        .collect();
    assert_eq!(new_nodes.len(), 1);
    assert!(!new_nodes[0].read().unwrap().nodes_from.is_empty());
    assert!(!new_graph.node_children(&new_nodes[0]).is_empty());
    assert!(new_graph.length() > graph.length());
}

#[test]
fn test_edge_mutation_for_graph() {
    // def test_edge_mutation_for_graph(graph):
    //     """
    //     Tests edge mutation can add edge between nodes
    //     """
    //     new_graph = deepcopy(graph)
    //     new_graph = single_edge_mutation(new_graph, **get_mutation_params())
    //     assert len(new_graph.get_edges()) > len(graph.get_edges())
    let graph = edge_mutation_graph_fixture();
    let rng = GeneticRng::seeded(0);
    let MutationParams {
        requirements,
        graph_gen_params,
        parameters,
    } = get_mutation_params(None, None, 1.0);
    let new_graph = single_edge_mutation(
        graph.deep_clone(),
        &requirements,
        &graph_gen_params,
        &parameters,
        &rng,
    );
    assert!(new_graph.get_edges().len() > graph.get_edges().len());
}

#[test]
fn test_replace_mutation() {
    // def test_replace_mutation(graph):
    //     """
    //     Tests single_change mutation can change node to another
    //     """
    //     new_graph = single_change_mutation(graph, **get_mutation_params())
    //     operations = [node.content['name'] for node in new_graph.nodes]
    //
    //     assert np.all([operation in available_node_types for operation in operations])
    let rng = GeneticRng::seeded(42);
    let graph = graph_fixture();
    let MutationParams {
        requirements,
        graph_gen_params,
        parameters,
    } = get_mutation_params(None, None, 1.0);
    let new_graph =
        single_change_mutation(graph, &requirements, &graph_gen_params, &parameters, &rng);
    for node in new_graph.nodes() {
        let name = node.read().unwrap().content.name.clone();
        assert!(AVAILABLE_NODE_TYPES.contains(&name.as_str()));
    }
}

#[test]
fn test_mutation_with_single_node() {
    // def test_mutation_with_single_node():
    //     adapter = DirectAdapter()
    //     graph = adapter.adapt(graph_with_single_node())
    //     new_graph = deepcopy(graph)
    //     params = get_mutation_params()
    //
    //     new_graph = reduce_mutation(new_graph, **params)
    //
    //     assert graph == new_graph
    //
    //     new_graph = single_drop_mutation(new_graph, **params)
    //
    //     assert graph == new_graph
    let adapter = DirectAdapter;
    let graph = adapter.adapt(graph_with_single_node());
    let rng = GeneticRng::seeded(0);
    let MutationParams {
        requirements,
        graph_gen_params,
        parameters,
    } = get_mutation_params(None, None, 1.0);
    let new_graph = reduce_mutation(
        (*graph).clone(),
        &requirements,
        &graph_gen_params,
        &parameters,
        &rng,
    );
    assert_eq!(new_graph, *graph);
    let new_graph = single_drop_mutation(
        new_graph,
        &requirements,
        &graph_gen_params,
        &parameters,
        &rng,
    );
    assert_eq!(new_graph, *graph);
}

#[test]
fn test_mutation_with_zero_prob() {
    // def test_mutation_with_zero_prob(mutation_type):
    //     adapter = DirectAdapter()
    //     params = get_mutation_params([mutation_type], mutation_prob=0)
    //     mutation = Mutation(**params)
    //
    //     ind = Individual(adapter.adapt(graph_first()))
    //     new_ind = mutation(ind)
    //
    //     assert new_ind.graph == ind.graph
    //     assert new_ind.uid == ind.uid
    //
    //     ind = Individual(adapter.adapt(graph_fifth()))
    //     new_ind = mutation(ind)
    //
    //     assert new_ind.graph == ind.graph
    //     assert new_ind.uid == ind.uid
    let adapter = DirectAdapter;
    let all_types = [
        MutationTypesEnum::Simple,
        MutationTypesEnum::Growth,
        MutationTypesEnum::LocalGrowth,
        MutationTypesEnum::TreeGrowth,
        MutationTypesEnum::Reduce,
        MutationTypesEnum::SingleAdd,
        MutationTypesEnum::SingleChange,
        MutationTypesEnum::SingleDrop,
        MutationTypesEnum::SingleEdge,
        MutationTypesEnum::None,
    ];
    for mutation_type in all_types {
        let MutationParams {
            requirements,
            graph_gen_params,
            parameters,
        } = get_mutation_params(Some(vec![mutation_type]), None, 0.0);
        let mutation = Mutation::new(parameters, requirements, graph_gen_params);

        let ind = Individual::new(adapter.adapt(graph_first()));
        let uid = ind.uid.clone();
        let graph_ptr = Arc::as_ptr(&ind.graph);
        match mutation.call(MutationTarget::Individual(ind)) {
            MutationResult::Individual(Some(new_ind)) => {
                assert_eq!(Arc::as_ptr(&new_ind.graph), graph_ptr);
                assert_eq!(new_ind.uid, uid);
            }
            _ => panic!("expected individual result"),
        }

        let ind = Individual::new(adapter.adapt(graph_fifth()));
        let uid = ind.uid.clone();
        let graph_ptr = Arc::as_ptr(&ind.graph);
        match mutation.call(MutationTarget::Individual(ind)) {
            MutationResult::Individual(Some(new_ind)) => {
                assert_eq!(Arc::as_ptr(&new_ind.graph), graph_ptr);
                assert_eq!(new_ind.uid, uid);
            }
            _ => panic!("expected individual result"),
        }
    }
}

#[test]
fn test_mutation_with_max_prob() {
    // def test_mutation_with_max_prob():
    //     """ Checks that individual is not included in next population if mutation was not applied
    //     due to inability to do this, not the probability  """
    //     adapter = DirectAdapter()
    //     params = get_mutation_params([MutationTypesEnum.reduce], mutation_prob=1)
    //     mutation = Mutation(**params)
    //
    //     ind = Individual(adapter.adapt(graph_with_single_node()))
    //     new_ind = mutation(ind)
    //     assert new_ind == []
    //
    //     population = [ind, ind]
    //     new_population = mutation(population)
    //     assert new_population == []
    let adapter = DirectAdapter;
    let MutationParams {
        requirements,
        graph_gen_params,
        parameters,
    } = get_mutation_params(Some(vec![MutationTypesEnum::Reduce]), None, 1.0);
    let mutation = Mutation::new(parameters, requirements, graph_gen_params);

    let ind = Individual::new(adapter.adapt(graph_with_single_node()));
    match mutation.call(MutationTarget::Individual(ind)) {
        MutationResult::Individual(None) => {}
        _ => panic!("expected None"),
    }

    let ind = Individual::new(adapter.adapt(graph_with_single_node()));
    match mutation.call(MutationTarget::Population(vec![ind.clone(), ind])) {
        MutationResult::Population(pop) => assert!(pop.is_empty()),
        _ => panic!("expected empty population"),
    }
}
