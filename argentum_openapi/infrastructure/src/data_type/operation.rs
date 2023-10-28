use crate::data_type::{RefOrObject, Reference, RequestBody, Response, SecurityRequirementObject};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub operation_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirementObject>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RefOrObject<RequestBody>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(default)]
    pub responses: BTreeMap<String, RefOrObject<Response>>,
}

impl Operation {
    pub fn set_request_body_as_ref(&mut self, reference: String) {
        self.request_body = Some(RefOrObject::Ref(Reference { reference }))
    }
}
