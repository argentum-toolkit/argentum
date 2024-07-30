use serde::{Serialize, Serializer};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct UnprocessableEntity {
    pub source: Box<dyn Error>,
}

impl Display for UnprocessableEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl UnprocessableEntity {
    pub fn new(source: Box<dyn Error>) -> Self {
        Self { source }
    }
}

impl Serialize for UnprocessableEntity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.source.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::error::UnprocessableEntity;

    #[test]
    fn test_serialize() {
        let ie = UnprocessableEntity::new(Box::new(ErrorMock {}));

        let actual = serde_json::to_string(&ie).unwrap();

        assert_eq!("\"test error\"", actual)
    }

    #[derive(thiserror::Error, Debug)]
    #[error("test error")]
    struct ErrorMock {}
}
