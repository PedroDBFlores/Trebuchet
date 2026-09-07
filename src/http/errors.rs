use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ResponseError {
    #[error("Connection error: {0}")]
    ConnectionError(#[from] std::io::Error),
    #[error("Timeout after {0}ms")]
    Timeout(u32),
}

#[cfg(test)]
#[path = "errors_test.rs"]
mod tests;
