use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    time::Duration,
};

use crate::tester::target::LoadTestTarget;

pub(crate) struct HttpTarget {
    pub(crate) target: LoadTestTarget,
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) method: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Option<String>,
    pub(crate) desired_status: u16,
}

impl HttpTarget {
    pub fn new(
        id: String,
        timeout: Duration,
        max_retries: u32,
        name: String,
        url: String,
        method: String,
        headers: HashMap<String, String>,
        body: Option<String>,
        desired_status: u16,
    ) -> Self {
        Self {
            target: LoadTestTarget::new(id, timeout, max_retries),
            name,
            url,
            method,
            headers,
            body,
            desired_status,
        }
    }
}

impl Deref for HttpTarget {
    type Target = LoadTestTarget;

    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

impl DerefMut for HttpTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.target
    }
}
