use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityRequirementObject {
    #[serde(flatten)]
    value: BTreeMap<String, Vec<String>>,
}
