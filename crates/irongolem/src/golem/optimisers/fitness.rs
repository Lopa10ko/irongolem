use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

/// Less is better — minimisation on raw metric values.
pub fn is_metric_worse(left: &[f64], right: &[f64]) -> bool {
    for (l, r) in left.iter().zip(right.iter()) {
        if *l > *r {
            return true;
        }
        if *l < *r {
            return false;
        }
    }
    left.len() > right.len()
}

fn round8(v: f64) -> f64 {
    (v * 1e8).round() / 1e8
}

fn allclose(values1: &[f64], values2: &[f64]) -> bool {
    if values1.len() != values2.len() {
        return false;
    }
    values1
        .iter()
        .zip(values2)
        .all(|(a, b)| (a - b).abs() < 1e-10 || (a - b).abs() / a.abs().max(1e-10) < 1e-8)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleObjFitness {
    values: Vec<Option<f64>>,
}

impl SingleObjFitness {
    pub fn new(primary: Option<f64>, supplementary: &[f64]) -> Self {
        let mut values = vec![primary];
        values.extend(supplementary.iter().map(|&v| Some(v)));
        Self { values }
    }

    pub fn from_values(values: Vec<Option<f64>>) -> Self {
        Self { values }
    }

    pub fn raw_values(&self) -> &[Option<f64>] {
        &self.values
    }

    pub fn is_valid(&self) -> bool {
        self.values.first().and_then(|v| *v).is_some()
    }

    pub fn values(&self) -> Vec<f64> {
        self.values.iter().map(|v| v.unwrap_or(0.0)).collect()
    }

    pub fn set_values(&mut self, new_values: Vec<Option<f64>>) {
        if new_values.is_empty() {
            self.values = vec![None];
            return;
        }
        if new_values.iter().skip(1).any(|v| v.is_none()) {
            panic!("Secondary values must not be None for prioritized fitness");
        }
        self.values = new_values;
    }

    pub fn clear_values(&mut self) {
        self.values = vec![None];
    }

    pub fn weights(&self) -> Vec<f64> {
        vec![1.0; self.values.len()]
    }

    fn cmp_lex(&self, other: &Self) -> Option<Ordering> {
        if !self.is_valid() {
            return Some(Ordering::Less);
        }
        if !other.is_valid() {
            return Some(Ordering::Greater);
        }
        let a = self.values();
        let b = other.values();
        if is_metric_worse(&a, &b) {
            Some(Ordering::Less)
        } else if is_metric_worse(&b, &a) {
            Some(Ordering::Greater)
        } else if allclose(&a, &b) {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl PartialEq for SingleObjFitness {
    fn eq(&self, other: &Self) -> bool {
        self.is_valid() && other.is_valid() && allclose(&self.values(), &other.values())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiObjFitness {
    wvalues: Vec<f64>,
    weights: Vec<f64>,
}

impl MultiObjFitness {
    pub fn new(values: &[f64], weights: Option<&[f64]>) -> Self {
        let weights: Vec<f64> = match weights {
            Some(w) if w.len() == values.len() => w.to_vec(),
            Some(w) if w.len() == 1 && !values.is_empty() => vec![w[0]; values.len()],
            Some(w) => {
                let mut padded = w.to_vec();
                padded.resize(values.len(), 1.0);
                padded
            }
            None => vec![1.0; values.len()],
        };
        let wvalues: Vec<f64> = values
            .iter()
            .zip(weights.iter())
            .map(|(v, w)| v * w)
            .collect();
        Self { wvalues, weights }
    }

    pub fn from_wvalues(wvalues: Vec<f64>, weights: Vec<f64>) -> Self {
        Self { wvalues, weights }
    }

    pub fn is_valid(&self) -> bool {
        !self.wvalues.is_empty()
    }

    pub fn values(&self) -> &[f64] {
        &self.wvalues
    }

    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    pub fn set_values(&mut self, values: &[f64]) {
        self.wvalues = values
            .iter()
            .zip(self.weights.iter())
            .map(|(v, w)| v * w)
            .collect();
    }

    pub fn clear_values(&mut self) {
        self.wvalues.clear();
    }

    pub fn dominates(&self, other: &MultiObjFitness) -> bool {
        if !self.is_valid() || !other.is_valid() {
            return false;
        }
        let mut not_equal = false;
        for (s, o) in self.wvalues.iter().zip(other.wvalues.iter()) {
            if is_metric_worse(std::slice::from_ref(o), std::slice::from_ref(s)) {
                not_equal = true;
            } else if is_metric_worse(std::slice::from_ref(s), std::slice::from_ref(o)) {
                return false;
            }
        }
        not_equal
    }

    fn cmp_lex(&self, other: &Self) -> Option<Ordering> {
        if !self.is_valid() {
            return Some(Ordering::Less);
        }
        if !other.is_valid() {
            return Some(Ordering::Greater);
        }
        if is_metric_worse(&self.wvalues, &other.wvalues) {
            Some(Ordering::Less)
        } else if is_metric_worse(&other.wvalues, &self.wvalues) {
            Some(Ordering::Greater)
        } else if allclose(&self.wvalues, &other.wvalues) {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl PartialEq for MultiObjFitness {
    fn eq(&self, other: &Self) -> bool {
        self.is_valid() && other.is_valid() && allclose(&self.wvalues, &other.wvalues)
    }
}

impl Hash for MultiObjFitness {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for v in &self.wvalues {
            round8(*v).to_bits().hash(state);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Fitness {
    #[default]
    Invalid,
    Single(SingleObjFitness),
    Multi(MultiObjFitness),
}

impl Fitness {
    pub fn is_valid(&self) -> bool {
        match self {
            Fitness::Invalid => false,
            Fitness::Single(s) => s.is_valid(),
            Fitness::Multi(m) => m.is_valid(),
        }
    }

    pub fn valid_fitness() -> Self {
        Fitness::Single(SingleObjFitness::new(Some(0.0), &[]))
    }

    pub fn values(&self) -> Vec<f64> {
        match self {
            Fitness::Invalid => vec![],
            Fitness::Single(s) => s.values(),
            Fitness::Multi(m) => m.values().to_vec(),
        }
    }

    pub fn set_values(&mut self, values: Vec<f64>) {
        match self {
            Fitness::Single(s) => s.set_values(values.into_iter().map(Some).collect()),
            Fitness::Multi(m) => m.set_values(&values),
            Fitness::Invalid => {
                *self = Fitness::Multi(MultiObjFitness::new(&values, None));
            }
        }
    }

    pub fn clear_values(&mut self) {
        match self {
            Fitness::Invalid => {}
            Fitness::Single(s) => s.clear_values(),
            Fitness::Multi(m) => m.clear_values(),
        }
    }

    pub fn weights(&self) -> Vec<f64> {
        match self {
            Fitness::Invalid => vec![],
            Fitness::Single(s) => s.weights(),
            Fitness::Multi(m) => m.weights().to_vec(),
        }
    }

    pub fn dominates(&self, other: &Fitness) -> bool {
        match (self, other) {
            (Fitness::Multi(a), Fitness::Multi(b)) => a.dominates(b),
            _ => self.partial_cmp(other) == Some(Ordering::Greater),
        }
    }

    fn cmp(&self, other: &Fitness) -> Option<Ordering> {
        match (self, other) {
            (Fitness::Single(a), Fitness::Single(b)) => a.cmp_lex(b),
            (Fitness::Multi(a), Fitness::Multi(b)) => a.cmp_lex(b),
            (Fitness::Invalid, Fitness::Invalid) => Some(Ordering::Equal),
            (Fitness::Invalid, _) => Some(Ordering::Less),
            (_, Fitness::Invalid) => Some(Ordering::Greater),
            _ if self.is_valid() && !other.is_valid() => Some(Ordering::Greater),
            _ if !self.is_valid() && other.is_valid() => Some(Ordering::Less),
            _ => None,
        }
    }
}

impl PartialEq for Fitness {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Fitness::Single(a), Fitness::Single(b)) => a == b,
            (Fitness::Multi(a), Fitness::Multi(b)) => a == b,
            (Fitness::Invalid, Fitness::Invalid) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Fitness {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other)
    }
}

impl Hash for Fitness {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Fitness::Invalid => 0u8.hash(state),
            Fitness::Single(s) => {
                1u8.hash(state);
                for v in s.values() {
                    round8(v).to_bits().hash(state);
                }
            }
            Fitness::Multi(m) => {
                2u8.hash(state);
                m.hash(state);
            }
        }
    }
}

pub fn null_fitness() -> Fitness {
    Fitness::Single(SingleObjFitness::new(None, &[]))
}

pub fn to_fitness(values: &[f64], multi_objective: bool) -> Fitness {
    if multi_objective {
        Fitness::Multi(MultiObjFitness::new(values, None))
    } else {
        let primary = values.first().copied();
        let supp: Vec<f64> = values.iter().skip(1).copied().collect();
        Fitness::Single(SingleObjFitness::new(primary, &supp))
    }
}

pub fn fitness_objects() -> Vec<Fitness> {
    vec![
        Fitness::Single(SingleObjFitness::new(Some(1.0), &[2.0, 3.0])),
        Fitness::Single(SingleObjFitness::new(None, &[10.0])),
        Fitness::Multi(MultiObjFitness::new(&[1.0, 2.0], None)),
        Fitness::Multi(MultiObjFitness::new(&[], None)),
        Fitness::Multi(MultiObjFitness::new(
            &[1.0, 2.0, 3.0],
            Some(&[1.0, -1.0, 1.0]),
        )),
    ]
}
