use super::HttpStatus;
use std::ops::{Deref, DerefMut};

use crate::tester::LoadTestResult;

pub(crate) struct HttpResult {
    pub(crate) result: LoadTestResult,
    pub(crate) status: HttpStatus,
    pub(crate) body: String,
}

impl Deref for HttpResult {
    type Target = LoadTestResult;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl DerefMut for HttpResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.result
    }
}
