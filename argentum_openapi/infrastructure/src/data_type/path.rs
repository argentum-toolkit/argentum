use crate::data_type::{Operation, Parameter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Self::Post => "POST".to_string(),
            Self::Get => "GET".to_string(),
            Self::Put => "PUT".to_string(),
            Self::Delete => "DELETE".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Path {
    #[serde(flatten)]
    pub operations: HashMap<Method, Operation>,

    pub parameters: Option<Vec<Parameter>>,
}
