use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Security {
    #[serde(flatten)]
    value: BTreeMap<String, Vec<String>>,
}
