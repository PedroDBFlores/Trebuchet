use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::tester::target::LoadTestTarget;

pub(crate) struct HttpTarget {
    pub(crate) target: LoadTestTarget,
    pub(crate) name: Box<str>,
    pub(crate) url: Box<str>,
    pub(crate) method: Box<str>,
    pub(crate) headers: HashMap<Box<str>, Box<str>>,
    pub(crate) body: Option<Box<str>>,
    pub(crate) desired_status: u16,
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
