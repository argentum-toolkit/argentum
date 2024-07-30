use crate::data_type::{RequestBody, Response, Schema, SecuritySchemeObject};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    #[serde(default)]
    pub request_bodies: BTreeMap<String, RequestBody>,

    #[serde(default)]
    pub schemas: BTreeMap<String, Schema>,

    #[serde(default)]
    pub responses: BTreeMap<String, Response>,

    #[serde(default)]
    pub security_schemes: BTreeMap<String, SecuritySchemeObject>,
}
