use serde::{Serialize, Serializer};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug)]
pub struct InternalServerError {
    pub source: Arc<dyn Error + Send + Sync>,
}

impl InternalServerError {
    pub fn new(source: Arc<dyn Error + Send + Sync>) -> Self {
        Self { source }
    }
}

impl Serialize for InternalServerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.source.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, sync::Arc};

    use crate::data_type::error::InternalServerError;

    #[test]
    fn test_serialize() -> Result<(), Box<dyn Error>> {
        let ie = InternalServerError::new(Arc::new(ErrorMock {}));

        let actual = serde_json::to_string(&ie)?;

        assert_eq!("\"test error\"", actual);

        Ok(())
    }

    #[derive(thiserror::Error, Debug)]
    #[error("test error")]
    struct ErrorMock {}
}
