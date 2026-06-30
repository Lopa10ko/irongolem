use std::cmp::Ordering;
use std::collections::HashMap;

use super::operator::{OperatorBase, PopulationT};
use crate::golem::optimisers::genetic::params::{GPAlgorithmParameters, SelectionType};
use crate::golem::optimisers::genetic::rng::GeneticRng;
use crate::golem::optimisers::genetic::GraphRequirements;
use crate::golem::optimisers::history::Individual;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionTypesEnum {
    Tournament,
    Spea2,
}

#[derive(Debug, Clone)]
pub struct Selection {
    base: OperatorBase,
}

impl Selection {
    pub fn new(parameters: GPAlgorithmParameters, requirements: GraphRequirements) -> Self {
        Self {
            base: OperatorBase::new(parameters, requirements),
        }
    }

    pub fn call(&self, population: PopulationT, pop_size: Option<usize>) -> PopulationT {
        let pop_size = pop_size.unwrap_or(self.base.parameters.pop_size);
        let rng = &self.base.rng;
        let selection_type = rng
            .random_choice(&self.base.parameters.selection_types)
            .unwrap_or(SelectionType::Tournament);
        match selection_type {
            SelectionType::Tournament => tournament_selection(population, pop_size, rng),
            SelectionType::Spea2 => spea2_selection(population, pop_size, rng),
            SelectionType::Custom(f) => f(&population, pop_size),
        }
    }
}

fn dedupe_by_uid(population: PopulationT) -> PopulationT {
    let mut map: HashMap<String, Individual> = HashMap::new();
    for ind in population {
        map.insert(ind.uid.clone(), ind);
    }
    map.into_values().collect()
}

pub fn tournament_selection(
    mut individuals: PopulationT,
    pop_size: usize,
    rng: &GeneticRng,
) -> PopulationT {
    individuals = dedupe_by_uid(individuals);
    if individuals.len() == 1 {
        return std::iter::repeat(individuals[0].clone())
            .take(pop_size)
            .collect();
    }
    if individuals.len() <= pop_size {
        return individuals;
    }

    let group_size = ((individuals.len() as f64) * 0.1).ceil() as usize;
    let group_size = group_size.max(2).min(individuals.len());
    let mut chosen = Vec::new();
    let iterations_limit = pop_size * 10;
    for _ in 0..iterations_limit {
        if chosen.len() >= pop_size {
            break;
        }
        let group = rng.sample(&individuals, group_size.min(individuals.len()));
        let best = group
            .into_iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap_or(Ordering::Equal))
            .unwrap();
        individuals.retain(|ind| ind.uid != best.uid);
        chosen.push(best);
    }
    chosen
}

pub fn random_selection(population: PopulationT, pop_size: usize, rng: &GeneticRng) -> PopulationT {
    let population = dedupe_by_uid(population);
    if population.len() == 1 {
        return std::iter::repeat(population[0].clone())
            .take(pop_size)
            .collect();
    }
    if population.len() <= pop_size {
        return population;
    }
    rng.sample(&population, pop_size)
}

pub fn spea2_selection(individuals: PopulationT, pop_size: usize, rng: &GeneticRng) -> PopulationT {
    let individuals = dedupe_by_uid(individuals);
    if individuals.len() == 1 {
        return std::iter::repeat(individuals[0].clone())
            .take(pop_size)
            .collect();
    }
    if individuals.len() <= pop_size {
        return individuals;
    }

    let inds_len = individuals.len();
    let fitness_len = individuals[0].fitness.values().len().max(1);
    let inds_len_sqrt = (inds_len as f64).sqrt();
    let mut strength_fits = vec![0usize; inds_len];
    let mut fits = vec![0.0f64; inds_len];
    let mut dominating_inds: Vec<Vec<usize>> = vec![Vec::new(); inds_len];

    for i in 0..inds_len {
        for j in (i + 1)..inds_len {
            if individuals[i].fitness.dominates(&individuals[j].fitness) {
                strength_fits[i] += 1;
                dominating_inds[j].push(i);
            } else if individuals[j].fitness.dominates(&individuals[i].fitness) {
                strength_fits[j] += 1;
                dominating_inds[i].push(j);
            }
        }
    }

    for i in 0..inds_len {
        for &j in &dominating_inds[i] {
            fits[i] += strength_fits[j] as f64;
        }
    }

    let mut chosen_indices: Vec<usize> = (0..inds_len).filter(|&i| fits[i] < 1.0).collect();

    match chosen_indices.len().cmp(&pop_size) {
        Ordering::Less => {
            for i in 0..inds_len {
                let mut distances = vec![0.0f64; inds_len];
                for j in (i + 1)..inds_len {
                    let mut dist = 0.0;
                    let vi = individuals[i].fitness.values();
                    let vj = individuals[j].fitness.values();
                    for idx in 0..fitness_len {
                        let a = vi.get(idx).copied().unwrap_or(0.0);
                        let b = vj.get(idx).copied().unwrap_or(0.0);
                        let val = a - b;
                        dist += val * val;
                    }
                    distances[j] = dist;
                }
                let kth = randomized_select(rng, &mut distances.clone(), 0, inds_len - 1, inds_len_sqrt);
                let density = 1.0 / (kth + 2.0);
                fits[i] += density;
            }
            let mut next_indices: Vec<(f64, usize)> = (0..inds_len)
                .filter(|i| !chosen_indices.contains(i))
                .map(|i| (fits[i], i))
                .collect();
            next_indices.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
            let need = pop_size - chosen_indices.len();
            chosen_indices.extend(next_indices.into_iter().take(need).map(|(_, i)| i));
        }
        Ordering::Greater => {
            trim_spea2_archive(&individuals, &mut chosen_indices, pop_size, fitness_len);
        }
        Ordering::Equal => {}
    }

    chosen_indices
        .into_iter()
        .map(|i| individuals[i].clone())
        .collect()
}

fn trim_spea2_archive(
    individuals: &[Individual],
    chosen_indices: &mut Vec<usize>,
    pop_size: usize,
    fitness_len: usize,
) {
    let inds_len = chosen_indices.len();
    let mut distances = vec![vec![0.0f64; inds_len]; inds_len];
    let mut sorted_indices = vec![vec![0usize; inds_len]; inds_len];

    for i in 0..inds_len {
        for j in (i + 1)..inds_len {
            let mut dist = 0.0;
            let vi = individuals[chosen_indices[i]].fitness.values();
            let vj = individuals[chosen_indices[j]].fitness.values();
            for idx in 0..fitness_len {
                let val = vi.get(idx).copied().unwrap_or(0.0) - vj.get(idx).copied().unwrap_or(0.0);
                dist += val * val;
            }
            distances[i][j] = dist;
            distances[j][i] = dist;
        }
        distances[i][i] = -1.0;
    }

    for i in 0..inds_len {
        for j in 1..inds_len {
            let mut idx = j;
            while idx > 0 && distances[i][j] < distances[i][sorted_indices[i][idx - 1]] {
                sorted_indices[i][idx] = sorted_indices[i][idx - 1];
                idx -= 1;
            }
            sorted_indices[i][idx] = j;
        }
    }

    let mut size = inds_len;
    let mut to_remove = Vec::new();
    while size > pop_size {
        let mut min_pos = 0;
        for i in 1..inds_len {
            for j in 1..size {
                let dist_i = distances[i][sorted_indices[i][j]];
                let dist_min = distances[min_pos][sorted_indices[min_pos][j]];
                if dist_i < dist_min {
                    min_pos = i;
                    break;
                } else if dist_i > dist_min {
                    break;
                }
            }
        }

        for i in 0..inds_len {
            distances[i][min_pos] = f64::INFINITY;
            distances[min_pos][i] = f64::INFINITY;
            for j in 1..size - 1 {
                if sorted_indices[i][j] == min_pos {
                    sorted_indices[i][j] = sorted_indices[i][j + 1];
                    sorted_indices[i][j + 1] = min_pos;
                }
            }
        }

        to_remove.push(min_pos);
        size -= 1;
    }

    to_remove.sort_unstable();
    for &index in to_remove.iter().rev() {
        chosen_indices.remove(index);
    }
}

fn randomized_select(
    rng: &GeneticRng,
    array: &mut [f64],
    begin: usize,
    end: usize,
    i: f64,
) -> f64 {
    if begin == end {
        return array[begin];
    }
    let q = randomized_partition(rng, array, begin, end);
    let k = q - begin + 1;
    if i < k as f64 {
        randomized_select(rng, array, begin, q, i)
    } else {
        randomized_select(rng, array, q + 1, end, i - k as f64)
    }
}

fn randomized_partition(rng: &GeneticRng, array: &mut [f64], begin: usize, end: usize) -> usize {
    let i = rng.gen_range(begin..=end);
    array.swap(begin, i);
    partition(array, begin, end)
}

fn partition(array: &mut [f64], begin: usize, end: usize) -> usize {
    let x = array[begin];
    let mut i = begin as isize - 1;
    let mut j = end as isize + 1;
    loop {
        loop {
            j -= 1;
            if array[j as usize] <= x {
                break;
            }
        }
        loop {
            i += 1;
            if array[i as usize] >= x {
                break;
            }
        }
        if i < j {
            array.swap(i as usize, j as usize);
        } else {
            return j as usize;
        }
    }
}
