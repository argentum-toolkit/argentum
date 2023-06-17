#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct Operation {
    pub id: String,
}

impl Operation {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
