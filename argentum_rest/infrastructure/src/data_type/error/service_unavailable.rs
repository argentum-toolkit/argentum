use serde::{Serialize, Serializer};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug)]
pub struct ServiceUnavailableError {
    pub source: Arc<dyn Error + Send + Sync>,
}

impl ServiceUnavailableError {
    pub fn new(source: Arc<dyn Error + Send + Sync>) -> Self {
        Self { source }
    }
}

impl Serialize for ServiceUnavailableError {
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

    use crate::data_type::error::ServiceUnavailableError;

    #[test]
    fn test_serialize() -> Result<(), Box<dyn Error>> {
        let e = ServiceUnavailableError::new(Arc::new(ErrorMock {}));

        let actual = serde_json::to_string(&e)?;

        assert_eq!("\"test error\"", actual);

        Ok(())
    }

    #[derive(thiserror::Error, Debug)]
    #[error("test error")]
    struct ErrorMock {}
}
