use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NotImplementedError {}

impl NotImplementedError {
    pub fn new() -> Self {
        Self {}
    }
}
