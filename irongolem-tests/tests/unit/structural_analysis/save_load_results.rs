//! save_load_results

#[test]
#[ignore = "deferred: structural_analysis out of scope"]
fn test_save_sa_results() {
    // def test_save_sa_results():
    //     opt_graph = OptGraph(OptNode('node1'))
    //
    //     objective = Objective(
    //         quality_metrics={
    //             'quality_custom_1': quality_custom_metric_1,
    //         }
    //     )
    //
    //     node_factory = DefaultOptNodeFactory()
    //
    //     requirements = StructuralAnalysisRequirements(graph_verifier=GraphVerifier(DEFAULT_DAG_RULES),
    //                                                   main_metric_idx=0,
    //                                                   seed=1)
    //
    //     # structural analysis will optimize given graph if at least one of the metrics was increased.
    //     sa = GraphStructuralAnalysis(objective=objective, node_factory=node_factory,
    //                                  requirements=requirements)
    //
    //     graph, results = sa.optimize(graph=opt_graph, n_jobs=1, max_iter=1)
    //
    //     path_to_save = os.path.join(TEST_FILE_NAME)
    //     saved_result = results.save(path=path_to_save, datetime_in_path=False)
    //
    //     assert TEST_FILE_NAME in os.listdir()
    //     assert saved_result is not None
    assert!(false);
}

#[test]
#[ignore = "deferred: structural_analysis out of scope"]
fn test_load_sa_results() {
    // def test_load_sa_results():
    //     graph = get_opt_graph()
    //     path = os.path.join(TEST_FILE_NAME)
    //     result_load = SAAnalysisResults.load(source=path, graph=graph)
    //
    //     assert result_load is not None
    //     os.remove(TEST_FILE_NAME)
    assert!(false);
}
