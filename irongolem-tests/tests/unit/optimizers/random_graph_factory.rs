//! random_graph_factory

use irongolem::golem::optimisers::genetic::params::{GraphGenerationParams, GraphRequirements};
use irongolem::golem::optimisers::genetic::rng::GeneticRng;
use irongolem::golem::dag::Graph;

#[test]
fn test_gp_composer_random_graph_generation_looping() {
    // def test_gp_composer_random_graph_generation_looping(max_depth):
    //     """ Test checks DefaultRandomOptGraphFactory valid generation. """
    let max_depth = 5;
    let available_node_types: Vec<String> = ["a", "b", "c", "d", "e"]
        .into_iter()
        .map(String::from)
        .collect();
    let requirements = GraphRequirements {
        max_depth,
        max_arity: 4,
        ..Default::default()
    };
    let graph_gen_params = GraphGenerationParams::new(available_node_types.clone())
        .with_rng(GeneticRng::seeded(42));
    let factory = graph_gen_params.random_graph_factory.clone();

    let graphs: Vec<_> = (0..20)
        .map(|_| factory.generate(&requirements, None))
        .collect();

    for graph in &graphs {
        for node in graph.nodes() {
            let name = node.read().unwrap().content.name.clone();
            assert!(available_node_types.contains(&name));
        }
        assert!((graph_gen_params.verifier)(graph));
        assert!(graph.depth() <= requirements.max_depth);
    }

    let min_depth = (max_depth as f64 * 0.25).ceil() as usize;
    assert!(graphs.iter().any(|g| g.depth() >= min_depth));
}
