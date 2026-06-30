use crate::golem::optimisers::genetic::params::{GPAlgorithmParameters, GraphRequirements};
use crate::golem::optimisers::genetic::rng::GeneticRng;
use crate::golem::optimisers::history::Individual;

pub type PopulationT = Vec<Individual>;
pub type EvaluationOperator = std::sync::Arc<dyn Fn(PopulationT) -> PopulationT + Send + Sync>;

#[derive(Clone)]
pub struct OperatorBase {
    pub requirements: GraphRequirements,
    pub parameters: GPAlgorithmParameters,
    pub rng: GeneticRng,
}

impl OperatorBase {
    pub fn new(parameters: GPAlgorithmParameters, requirements: GraphRequirements) -> Self {
        let rng = GeneticRng::from_parameters(&parameters);
        Self {
            requirements,
            parameters,
            rng,
        }
    }
}

impl std::fmt::Debug for OperatorBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OperatorBase")
            .field("requirements", &self.requirements)
            .field(
                "parameters",
                &format!(
                    "GPAlgorithmParameters(pop_size={})",
                    self.parameters.pop_size
                ),
            )
            .field("rng", &self.rng)
            .finish()
    }
}
