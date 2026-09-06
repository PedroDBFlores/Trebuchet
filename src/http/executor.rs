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
        Self {
            client: ureq::Agent::new_with_defaults(),
        }
    }
}

impl TestExecutor<HttpTarget, HttpResult> for HttpExecutor {
    fn execute(&self, target: &HttpTarget) -> Result<HttpResult, Box<dyn std::error::Error>> {
        let current_time = std::time::Instant::now();
        let response = self.client.get(&target.url).call();
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
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use httpmock::MockServer;

    use crate::{
        http::{HttpStatus, executor::HttpExecutor, target::HttpTarget},
        tester::TestExecutor,
    };

    #[test]
    fn test_execute() {
        let server = MockServer::start();
        let handle = server.mock(|when, then| {
            when.method("GET").path("/");
            then.status(200).body("OK");
        });
        let executor = HttpExecutor::default();
        let target = HttpTarget::new(
            "id".to_string(),
            std::time::Duration::new(5, 0),
            1,
            "test".to_string(),
            server.base_url(),
            crate::http::HttpMethod::GET,
            HashMap::new(),
            None,
            200.into(),
        );
        let result = executor.execute(&target);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value.status, HttpStatus::Success(200));
        assert!(value.latency > std::time::Duration::new(0, 0));

        handle.assert();
    }
}
