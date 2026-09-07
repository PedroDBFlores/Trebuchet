use std::time::Duration;

#[derive(Debug, Clone)]
pub(crate) struct LoadTestTarget {
    pub(crate) id: String,
    pub(crate) timeout: Duration,
    pub(crate) max_retries: u32,
}

impl LoadTestTarget {
    pub(crate) fn new(id: String, timeout: Duration, max_retries: u32) -> Self {
        Self {
            id,
            timeout,
            max_retries,
        }
    }
}
