use std::time::Duration;

pub struct OptimisationTimer {
    pub timeout: Duration,
}

impl OptimisationTimer {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}
