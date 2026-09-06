use super::target::LoadTestTarget;

pub(crate) struct LoadTestConfig {
    id: String,
    targets: Vec<LoadTestTarget>,
}
