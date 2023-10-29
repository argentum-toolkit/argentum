use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecuritySchemeObject {
    #[serde(default)]
    pub scheme: String,

    #[serde(rename = "type", default)]
    pub security_scheme_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default)]
    pub name: String,

    #[serde(rename = "in", default)]
    pub in_location: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_format: Option<String>,

    //TODO: add field flows
    #[serde(default)]
    pub open_id_connect_url: String,
}
