#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct Request {
    pub body_schema: String,
    pub security: bool,
}

impl Request {
    pub fn new(body_schema: String, security: bool) -> Self {
        Self {
            body_schema,
            security,
        }
    }
}
