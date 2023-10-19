use crate::data_type::{MediaTypeObject, Schema};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    #[serde(default)]
    pub content: BTreeMap<String, MediaTypeObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
