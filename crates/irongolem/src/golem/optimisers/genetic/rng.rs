use std::sync::{Arc, Mutex};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::params::GPAlgorithmParameters;

thread_local! {
    static DEFAULT_RNG: Mutex<StdRng> = Mutex::new(StdRng::from_entropy());
}

/// Injectable RNG for genetic operators (clone shares state via `Arc`).
#[derive(Clone)]
pub struct GeneticRng {
    inner: Arc<Mutex<StdRng>>,
}

impl GeneticRng {
    pub fn seeded(seed: u64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(StdRng::seed_from_u64(seed))),
        }
    }

    pub fn entropy() -> Self {
        Self {
            inner: Arc::new(Mutex::new(StdRng::from_entropy())),
        }
    }

    pub fn from_parameters(parameters: &GPAlgorithmParameters) -> Self {
        parameters
            .random_seed
            .map(Self::seeded)
            .unwrap_or_else(Self::entropy)
    }

    pub fn with_rng<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut StdRng) -> R,
    {
        f(&mut self.inner.lock().unwrap())
    }

    pub fn gen_f64(&self) -> f64 {
        self.with_rng(|rng| rng.gen())
    }

    pub fn gen_range<R>(&self, range: R) -> usize
    where
        R: rand::distributions::uniform::SampleRange<usize>,
    {
        self.with_rng(|rng| rng.gen_range(range))
    }

    pub fn random_index(&self, len: usize) -> usize {
        if len == 0 {
            0
        } else {
            self.gen_range(0..len)
        }
    }

    pub fn random_choice<T: Clone>(&self, items: &[T]) -> Option<T> {
        if items.is_empty() {
            None
        } else {
            Some(items[self.random_index(items.len())].clone())
        }
    }

    pub fn sample<T: Clone>(&self, items: &[T], count: usize) -> Vec<T> {
        if items.is_empty() || count == 0 {
            return Vec::new();
        }
        let count = count.min(items.len());
        self.with_rng(|rng| {
            let mut indices: Vec<usize> = (0..items.len()).collect();
            for i in 0..count {
                let j = rng.gen_range(i..indices.len());
                indices.swap(i, j);
            }
            indices[..count].iter().map(|&i| items[i].clone()).collect()
        })
    }

    pub fn shuffle<T>(&self, items: &mut [T]) {
        self.with_rng(|rng| {
            for i in (1..items.len()).rev() {
                let j = rng.gen_range(0..=i);
                items.swap(i, j);
            }
        })
    }
}

impl std::fmt::Debug for GeneticRng {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("GeneticRng")
    }
}

pub fn set_random_seed(seed: u64) {
    DEFAULT_RNG.with(|rng| *rng.lock().unwrap() = StdRng::seed_from_u64(seed));
}

pub fn maybe_seed(parameters: &GPAlgorithmParameters) {
    if let Some(seed) = parameters.random_seed {
        set_random_seed(seed);
    }
}

fn with_default_rng<F, R>(f: F) -> R
where
    F: FnOnce(&mut StdRng) -> R,
{
    DEFAULT_RNG.with(|rng| f(&mut rng.lock().unwrap()))
}

pub fn with_rng<F, R>(f: F) -> R
where
    F: FnOnce(&mut StdRng) -> R,
{
    with_default_rng(f)
}

pub fn random_f64() -> f64 {
    with_default_rng(|rng| rng.gen())
}

pub fn random_index(len: usize) -> usize {
    if len == 0 {
        0
    } else {
        with_default_rng(|rng| rng.gen_range(0..len))
    }
}

pub fn random_choice<T: Clone>(items: &[T]) -> Option<T> {
    if items.is_empty() {
        None
    } else {
        Some(items[random_index(items.len())].clone())
    }
}

pub fn sample<T: Clone>(items: &[T], count: usize) -> Vec<T> {
    if items.is_empty() || count == 0 {
        return Vec::new();
    }
    let count = count.min(items.len());
    with_default_rng(|rng| {
        let mut indices: Vec<usize> = (0..items.len()).collect();
        for i in 0..count {
            let j = rng.gen_range(i..indices.len());
            indices.swap(i, j);
        }
        indices[..count].iter().map(|&i| items[i].clone()).collect()
    })
}

pub fn shuffle<T>(items: &mut [T]) {
    with_default_rng(|rng| {
        for i in (1..items.len()).rev() {
            let j = rng.gen_range(0..=i);
            items.swap(i, j);
        }
    })
}
