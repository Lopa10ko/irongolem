use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct OptimisationTimer {
    pub timeout: Option<Duration>,
    start: Option<Instant>,
    init_time_minutes: f64,
    pub process_terminated: bool,
}

impl OptimisationTimer {
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout: Some(timeout),
            start: None,
            init_time_minutes: 0.0,
            process_terminated: false,
        }
    }

    pub fn forever() -> Self {
        Self {
            timeout: None,
            start: None,
            init_time_minutes: 0.0,
            process_terminated: false,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
        self.process_terminated = false;
    }

    pub fn is_started(&self) -> bool {
        self.start.is_some()
    }

    pub fn minutes_from_start(&self) -> f64 {
        self.spent_time().as_secs_f64() / 60.0
    }

    pub fn spent_time(&self) -> Duration {
        self.start.map(|s| s.elapsed()).unwrap_or(Duration::ZERO)
    }

    pub fn set_init_time(&mut self, init_time_minutes: f64) {
        self.init_time_minutes = init_time_minutes;
    }

    fn is_next_iteration_possible(
        &mut self,
        time_constraint_minutes: f64,
        iteration_num: Option<usize>,
    ) -> bool {
        let minutes = self.minutes_from_start();
        let possible = if let Some(iter) = iteration_num {
            if iter != 0 {
                let evo_proc_minutes = minutes - self.init_time_minutes;
                time_constraint_minutes > minutes + (evo_proc_minutes / iter as f64)
            } else {
                time_constraint_minutes > minutes
            }
        } else {
            time_constraint_minutes > minutes
        };
        if !possible {
            self.process_terminated = true;
        }
        possible
    }

    pub fn is_time_limit_reached(&mut self, iteration_num: Option<usize>) -> bool {
        let Some(timeout) = self.timeout else {
            return false;
        };
        let timeout_minutes = if timeout.as_secs_f64() < 0.0 {
            0.0
        } else {
            timeout.as_secs_f64() / 60.0
        };
        if timeout_minutes == 0.0 {
            self.process_terminated = true;
            return true;
        }
        !self.is_next_iteration_possible(timeout_minutes, iteration_num)
    }
}

impl Default for OptimisationTimer {
    fn default() -> Self {
        Self::forever()
    }
}
