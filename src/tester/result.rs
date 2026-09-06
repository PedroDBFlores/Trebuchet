use std::{error::Error, time::Duration};

pub(crate) struct LoadTestResult {
    latency: Duration,
    error: Box<dyn Error>,
}
