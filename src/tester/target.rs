use std::time::Duration;

pub(crate) struct LoadTestTarget {
    id: Box<str>,
    timeout: Duration,
    max_retries: u32,
}
