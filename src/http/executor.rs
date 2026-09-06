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
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl TestExecutor<super::target::HttpTarget, super::result::HttpResult> for HttpExecutor {
    fn execute(&self, target: &super::target::HttpTarget) -> super::result::HttpResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::http::{executor::HttpExecutor, target::HttpTarget};

    #[test]
    fn test_execute() {
        let executor = HttpExecutor::default();
        let target = HttpTarget::default();
        let result = executor.execute(&target);
        assert!(result.is_ok());
    }
}
