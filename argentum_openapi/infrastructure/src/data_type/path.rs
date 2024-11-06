use super::Method;
use crate::data_type::{Operation, Parameter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Path {
    #[serde(flatten)]
    pub operations: HashMap<Method, Operation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}
