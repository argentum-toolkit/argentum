use crate::description::Request;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct Operation {
    pub id: String,
    pub request: Option<Request>,
}

impl Operation {
    pub fn new(id: String, request: Option<Request>) -> Self {
        Self { id, request }
    }
}
