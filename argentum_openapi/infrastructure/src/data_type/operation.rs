use crate::data_type::request_body::RefOrObject;
use crate::data_type::{RequestBody, Security};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub operation_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<Security>>,

    pub request_body: Option<RefOrObject<RequestBody>>,
}
