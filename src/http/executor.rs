use super::HttpResult;
use super::HttpTarget;
use crate::tester::TestExecutor;

pub(crate) struct HttpExecutor {
    client: ureq::Agent,
}

impl HttpExecutor {
    pub fn new(client: ureq::Agent) -> Self {
        Self { client }
    }
}

impl Default for HttpExecutor {
    fn default() -> Self {
        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .build();
        Self {
            client: ureq::Agent::new_with_config(config),
        }
    }
}

impl TestExecutor<HttpTarget, HttpResult> for HttpExecutor {
    fn execute(&self, target: &HttpTarget) -> Result<HttpResult, Box<dyn std::error::Error>> {
        let mut request = self
            .client
            .get(&target.url)
            .config()
            .timeout_global(Some(target.timeout))
            .build();
        for (key, value) in &target.headers {
            request = request.header(key, value);
        }
        let current_time = std::time::Instant::now();
        let response = request.call();
        let elapsed = current_time.elapsed();
        match response {
            Ok(mut response) => {
                let status = response.status();
                let body = response.body_mut();
                let string_body = body.read_to_string();
                Ok(HttpResult {
                    result: crate::tester::LoadTestResult { latency: elapsed },
                    status: status.as_u16().into(),
                    body: string_body.unwrap_or_default(),
                })
            }
            Err(ureq::Error::Timeout(_)) => Err(Box::new(super::errors::ResponseError::Timeout(
                target.timeout.as_millis() as u32,
            ))),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[cfg(test)]
#[path = "executor_test.rs"]
mod tests;
