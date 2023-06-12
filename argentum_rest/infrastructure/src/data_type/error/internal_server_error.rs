use serde::{Serialize, Serializer};
use std::error::Error;

#[derive(Debug)]
pub struct InternalServerError {
    pub source: Box<dyn Error>,
}

impl InternalServerError {
    pub fn new(source: Box<dyn Error>) -> Self {
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
    use crate::data_type::error::InternalServerError;

    #[test]
    fn test_serialize() {
        let ie = InternalServerError::new(Box::new(ErrorMock {}));

        let actual = serde_json::to_string(&ie).unwrap();

        assert_eq!("\"test error\"", actual)
    }

    #[derive(thiserror::Error, Debug)]
    #[error("test error")]
    struct ErrorMock {}
}
