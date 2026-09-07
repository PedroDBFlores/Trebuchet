use std::time::Duration;

#[derive(Debug)]
pub(crate) struct LoadTestResult {
    pub(crate) latency: Duration,
}
