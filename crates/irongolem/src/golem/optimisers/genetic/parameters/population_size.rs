use std::cell::RefCell;
use std::rc::Rc;

use super::generation_keeper::ImprovementWatcher;
use super::parameter::AdaptiveParameter;
use super::sequence_iterator::SequenceIterator;
use crate::golem::optimisers::genetic::constants::MIN_POP_SIZE;
use crate::golem::optimisers::genetic::operators::PopulationT;

pub type PopulationSize = dyn AdaptiveParameter<usize>;

pub struct ConstRatePopulationSize {
    offspring_rate: f64,
    initial: usize,
    max_pop_size: Option<usize>,
}

impl ConstRatePopulationSize {
    pub fn new(pop_size: usize, offspring_rate: f64, max_pop_size: Option<usize>) -> Self {
        Self {
            offspring_rate,
            initial: pop_size,
            max_pop_size,
        }
    }
}

impl AdaptiveParameter<usize> for ConstRatePopulationSize {
    fn initial(&self) -> usize {
        self.initial
    }

    fn next(&self, population: &PopulationT) -> usize {
        let mut pop_size = population.len().max(self.initial);
        if self.max_pop_size.is_none() || pop_size < self.max_pop_size.unwrap() {
            pop_size += (pop_size as f64 * self.offspring_rate).ceil() as usize;
        }
        if let Some(max_pop) = self.max_pop_size {
            pop_size = pop_size.min(max_pop);
        }
        pop_size
    }
}

pub struct AdaptivePopulationSize {
    improvements: Rc<RefCell<dyn ImprovementWatcher>>,
    iterator: RefCell<SequenceIterator>,
    max_pop_size: Option<usize>,
    initial: usize,
}

impl AdaptivePopulationSize {
    pub fn new(
        improvement_watcher: Rc<RefCell<dyn ImprovementWatcher>>,
        progression_iterator: SequenceIterator,
        max_pop_size: Option<usize>,
    ) -> Self {
        let iterator = progression_iterator;
        let initial = if iterator.has_next() {
            iterator.next_value()
        } else {
            iterator.prev_value().unwrap_or_else(|| iterator.current())
        };
        Self {
            improvements: improvement_watcher,
            iterator: RefCell::new(iterator),
            max_pop_size,
            initial,
        }
    }
}

impl AdaptiveParameter<usize> for AdaptivePopulationSize {
    fn initial(&self) -> usize {
        self.initial
    }

    fn next(&self, population: &PopulationT) -> usize {
        let mut pop_size = population.len();
        let current = self.iterator.borrow().current();
        let too_many_fitness_eval_errors = (pop_size as f64) / (current as f64) < 0.5;

        let improvements = self.improvements.borrow();
        if too_many_fitness_eval_errors || !improvements.is_any_improved() {
            if self.iterator.borrow().has_next() {
                pop_size = self.iterator.borrow_mut().next_value();
            }
        } else if improvements.is_quality_improved()
            && improvements.is_complexity_improved()
            && pop_size > 0
            && self.iterator.borrow().has_prev()
        {
            if let Some(prev) = self.iterator.borrow_mut().prev_value() {
                pop_size = prev;
            }
        }

        pop_size = pop_size.max(MIN_POP_SIZE);
        if let Some(max_pop) = self.max_pop_size {
            pop_size = pop_size.min(max_pop);
        }
        pop_size
    }
}
