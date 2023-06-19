use crate::description::Operation;
use hyper::Method;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) struct Path {
    pub path: String,
    pub operations: HashMap<Method, Operation>,
}

impl Path {
    pub fn new(path: String, operations: HashMap<Method, Operation>) -> Self {
        Self { path, operations }
    }
}
