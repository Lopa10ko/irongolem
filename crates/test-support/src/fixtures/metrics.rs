use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::golem::dag::GraphDelegate;

pub struct RandomMetric;

impl RandomMetric {
    pub fn get_value(_graph: Arc<GraphDelegate>, delay: Duration) -> f64 {
        thread::sleep(delay);
        0.0
    }
}

pub struct DepthMetric;

impl DepthMetric {
    pub fn get_value(_graph: Arc<GraphDelegate>) -> f64 {
        0.0
    }
}

pub struct ParamsSumMetric;

impl ParamsSumMetric {
    pub fn get_value(_graph: Arc<GraphDelegate>) -> f64 {
        0.0
    }
}
