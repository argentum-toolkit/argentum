#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct Operation {
    pub id: String,
    pub request: bool,
    //TODO: security should depend on response
    pub security: bool,
}

impl Operation {
    pub fn new(id: String, request: bool, security: bool) -> Self {
        Self {
            id,
            request,
            security,
        }
    }
}
