use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::data_type::MediaTypeObject;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestBody {
    #[serde(default)]
    pub content: BTreeMap<String, MediaTypeObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
