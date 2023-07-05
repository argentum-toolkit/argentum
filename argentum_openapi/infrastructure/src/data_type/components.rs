use crate::data_type::RequestBody;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    pub request_bodies: BTreeMap<String, RequestBody>,
    // pub schemas: BTreeMap<String, String>,
}
