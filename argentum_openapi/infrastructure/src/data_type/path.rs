use crate::data_type::Operation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Post,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Post => "POST".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Path {
    #[serde(flatten)]
    pub operations: HashMap<Method, Operation>,
}
