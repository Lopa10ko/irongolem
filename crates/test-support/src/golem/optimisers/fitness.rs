#[derive(Debug, Clone, Default, PartialEq)]
pub struct Fitness {
    pub valid: bool,
}

impl Fitness {
    pub fn valid_fitness() -> Self {
        Self { valid: true }
    }

    pub fn null() -> Self {
        Self { valid: false }
    }
}

pub fn null_fitness() -> Fitness {
    Fitness::null()
}
