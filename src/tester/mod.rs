use std::ops::Deref;

mod config;
mod result;
mod target;

pub(crate) use config::LoadTestConfig;
pub(crate) use result::LoadTestResult;
pub(crate) use target::LoadTestTarget;

pub(crate) trait TestExecutor<
    T: Deref<Target = target::LoadTestTarget>,
    R: Deref<Target = result::LoadTestResult>,
>
{
    async fn execute(&self, target: &T) -> Result<R, Box<dyn std::error::Error>>;
}
