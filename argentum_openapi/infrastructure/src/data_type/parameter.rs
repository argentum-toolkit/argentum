use crate::data_type::{RefOrObject, Schema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,

    #[serde(rename = "in")]
    pub in_place: In,

    pub description: Option<String>,

    #[serde(default)]
    pub required: bool,

    #[serde(default)]
    pub deprecated: bool,

    #[serde(default)]
    pub allow_empty_value: bool,

    pub schema: RefOrObject<Schema>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum In {
    Query,
    Header,
    Path,
    Cookie,
}
