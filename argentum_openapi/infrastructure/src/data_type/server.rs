use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerVariableObject {
    default: String,

    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    enum_values: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    url: String,

    #[serde(default)]
    variables: BTreeMap<String, ServerVariableObject>,
}
