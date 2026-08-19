use std::sync::Arc;

use super::genetic::constants::MAX_GRAPH_GEN_ATTEMPTS;
use super::genetic::params::{GraphGenerationParams, GraphRequirements};
use crate::golem::dag::GraphDelegate;

pub type GenerationFn = Box<dyn FnMut() -> GraphDelegate + Send>;
pub type InitialGraphsGenerator = Box<dyn FnMut() -> Vec<Arc<GraphDelegate>> + Send>;

pub struct InitialPopulationGenerator {
    pub pop_size: usize,
    pub requirements: GraphRequirements,
    pub graph_generation_params: GraphGenerationParams,
    initial_graphs: Option<Vec<Arc<GraphDelegate>>>,
    generation_function: Option<GenerationFn>,
}

impl InitialPopulationGenerator {
    pub fn new(
        population_size: usize,
        generation_params: GraphGenerationParams,
        requirements: GraphRequirements,
    ) -> Self {
        Self {
            pop_size: population_size,
            requirements,
            graph_generation_params: generation_params,
            initial_graphs: None,
            generation_function: None,
        }
    }

    pub fn with_initial_graphs(mut self, initial_graphs: Vec<Arc<GraphDelegate>>) -> Self {
        self.initial_graphs = Some(initial_graphs);
        self
    }

    pub fn with_custom_generation_function(mut self, generation_func: GenerationFn) -> Self {
        self.generation_function = Some(generation_func);
        self
    }

    pub fn generate(&mut self) -> Vec<Arc<GraphDelegate>> {
        if let Some(ref graphs) = self.initial_graphs {
            let result: Vec<_> = graphs.iter().take(self.pop_size).cloned().collect();
            return result;
        }

        let verifier = self.graph_generation_params.verifier.clone();
        let mut population: Vec<Arc<GraphDelegate>> = Vec::new();

        if self.generation_function.is_none() {
            let factory = self.graph_generation_params.random_graph_factory.clone();
            let requirements = self.requirements.clone();
            for _ in 0..MAX_GRAPH_GEN_ATTEMPTS {
                if population.len() == self.pop_size {
                    break;
                }
                let new_graph = Arc::new(factory.generate(&requirements, None));
                if !population.iter().any(|g| g.as_ref() == new_graph.as_ref())
                    && verifier(&new_graph)
                {
                    population.push(new_graph);
                }
            }
            return population;
        }

        let mut generation_function = self.generation_function.take().unwrap();
        for _ in 0..MAX_GRAPH_GEN_ATTEMPTS {
            if population.len() == self.pop_size {
                break;
            }
            let new_graph = Arc::new(generation_function());
            if !population.iter().any(|g| g.as_ref() == new_graph.as_ref()) && verifier(&new_graph)
            {
                population.push(new_graph);
            }
        }
        self.generation_function = Some(generation_function);
        population
    }
}

impl InitialPopulationGenerator {
    /// Alias mirroring Python `__call__`.
    pub fn call(&mut self) -> Vec<Arc<GraphDelegate>> {
        self.generate()
    }
}
