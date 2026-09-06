use std::ops::Deref;

pub(crate) mod config;
pub(crate) mod result;
pub(crate) mod target;

pub(crate) trait TestExecutor<
    T: Deref<Target = target::LoadTestTarget>,
    R: Deref<Target = result::LoadTestResult>,
>
{
    fn execute(&self, target: &T) -> Result<R, Box<dyn std::error::Error>>;
}
