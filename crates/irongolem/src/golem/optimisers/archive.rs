use std::cmp::Ordering;

use crate::golem::optimisers::fitness::Fitness;
use crate::golem::optimisers::genetic::operators::PopulationT;
use crate::golem::optimisers::history::Individual;

fn individuals_same(a: &Individual, b: &Individual) -> bool {
    a.fitness == b.fitness && a.native_generation == b.native_generation && a.graph == b.graph
}

pub struct HallOfFame {
    pub maxsize: usize,
    pub items: Vec<Individual>,
    keys: Vec<Fitness>,
    similar: fn(&Individual, &Individual) -> bool,
}

impl HallOfFame {
    pub fn new(maxsize: usize) -> Self {
        Self {
            maxsize,
            items: Vec::new(),
            keys: Vec::new(),
            similar: individuals_same,
        }
    }

    pub fn with_similar(maxsize: usize, similar: fn(&Individual, &Individual) -> bool) -> Self {
        Self {
            maxsize,
            items: Vec::new(),
            keys: Vec::new(),
            similar,
        }
    }

    pub fn update(&mut self, population: &PopulationT) {
        if population.is_empty() {
            return;
        }
        for ind in population {
            if self.items.is_empty() && self.maxsize != 0 {
                self.insert(ind.clone());
                continue;
            }
            let worse_than_worst = self
                .items
                .last()
                .map(|last| ind.fitness.partial_cmp(&last.fitness) == Some(Ordering::Greater))
                .unwrap_or(true);
            if worse_than_worst || self.items.len() < self.maxsize {
                let has_similar = self.items.iter().any(|hofer| (self.similar)(ind, hofer));
                if !has_similar {
                    if self.items.len() >= self.maxsize {
                        self.remove_last();
                    }
                    self.insert(ind.clone());
                }
            }
        }
    }

    fn insert(&mut self, item: Individual) {
        let i = self
            .keys
            .binary_search_by(|k| k.partial_cmp(&item.fitness).unwrap_or(Ordering::Equal))
            .unwrap_or_else(|e| e);
        self.keys.insert(i, item.fitness.clone());
        self.items.insert(self.items.len() - i, item);
    }

    fn remove_last(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.keys.remove(0);
        self.items.pop();
    }
}

pub struct ParetoFront {
    pub maxsize: usize,
    pub items: Vec<Individual>,
    similar: fn(&Individual, &Individual) -> bool,
}

impl ParetoFront {
    pub fn new(maxsize: usize) -> Self {
        Self {
            maxsize,
            items: Vec::new(),
            similar: individuals_same,
        }
    }

    pub fn update(&mut self, population: &PopulationT) {
        for ind in population {
            let mut is_dominated = false;
            let mut dominates_one = false;
            let mut has_twin = false;
            let mut to_remove = Vec::new();

            for (i, hof_member) in self.items.iter().enumerate() {
                if !dominates_one && hof_member.fitness.dominates(&ind.fitness) {
                    is_dominated = true;
                    break;
                } else if ind.fitness.dominates(&hof_member.fitness) {
                    dominates_one = true;
                    to_remove.push(i);
                } else if ind.fitness == hof_member.fitness && (self.similar)(ind, hof_member) {
                    has_twin = true;
                    break;
                }
            }

            for i in to_remove.into_iter().rev() {
                self.items.remove(i);
            }

            if !is_dominated && !has_twin {
                if self.maxsize > 0 && self.items.len() >= self.maxsize {
                    self.items.pop();
                }
                self.items.push(ind.clone());
            }
        }
    }
}
