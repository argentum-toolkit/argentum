use crate::data_type::{Reference, RefOrObject, RequestBody, Security};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub operation_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<Security>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RefOrObject<RequestBody>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,


    //todo:tags
    // pub responses: Option<RefOrObject<RequestBody>>,
}

impl Operation {
    pub fn set_request_body_as_ref(&mut self, reference: String) {
        self.request_body = Some(RefOrObject::Ref(Reference{reference}))
    }
}
