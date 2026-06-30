use crate::golem::optimisers::genetic::operators::PopulationT;

pub trait AdaptiveParameter<T> {
    fn initial(&self) -> T;
    fn next(&self, population: &PopulationT) -> T;
}

pub struct ConstParameter<T: Clone> {
    value: T,
}

impl<T: Clone> ConstParameter<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Clone> AdaptiveParameter<T> for ConstParameter<T> {
    fn initial(&self) -> T {
        self.value.clone()
    }

    fn next(&self, _population: &PopulationT) -> T {
        self.value.clone()
    }
}
