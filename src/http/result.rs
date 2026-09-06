use std::ops::{Deref, DerefMut};

use crate::tester::result::LoadTestResult;

pub(super) struct HttpResult {
    result: LoadTestResult,
    status: u16,
    body: Box<str>,
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
