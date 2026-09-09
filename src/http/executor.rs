use super::HttpMethod;
use super::HttpResult;
use super::HttpTarget;
use crate::tester::TestExecutor;

pub(crate) struct HttpExecutor {
    client: reqwest::Client,
}

impl HttpExecutor {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl Default for HttpExecutor {
    fn default() -> Self {
        let client = reqwest::ClientBuilder::new().build().unwrap();
        Self { client }
    }
}

impl TestExecutor<HttpTarget, HttpResult> for HttpExecutor {
    async fn execute(&self, target: &HttpTarget) -> Result<HttpResult, Box<dyn std::error::Error>> {
        let current_time = std::time::Instant::now();

        let mut request_builder = match target.method {
            HttpMethod::GET => self.client.get(&target.url),
            HttpMethod::POST => self.client.post(&target.url),
            HttpMethod::PUT => self.client.put(&target.url),
            HttpMethod::DELETE => self.client.delete(&target.url),
        };

        request_builder = request_builder.timeout(target.timeout);

        for (key, value) in &target.headers {
            request_builder = request_builder.header(key, value);
        }

        if let Some(body) = &target.body {
            request_builder = request_builder.body(body.clone());
        }

        let response = request_builder.send().await;
        let elapsed = current_time.elapsed();

        match response {
            Ok(response) => {
                let status = response.status();
                let body = response.text().await?;
                Ok(HttpResult {
                    result: crate::tester::LoadTestResult { latency: elapsed },
                    status: status.as_u16().into(),
                    body,
                })
            }
            Err(e) => {
                if e.is_timeout() {
                    return Err(Box::new(super::errors::ResponseError::Timeout(
                        target.timeout.as_millis() as u32,
                    )));
                }
                Err(Box::new(e))
            }
        }
    }
}

#[cfg(test)]
#[path = "executor_test.rs"]
mod tests;
