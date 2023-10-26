use crate::data_type::{RefOrObject, Schema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Obj {
    #[serde(rename = "type")]
    pub obj_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaTypeObject {
    pub schema: RefOrObject<Schema>,
}
