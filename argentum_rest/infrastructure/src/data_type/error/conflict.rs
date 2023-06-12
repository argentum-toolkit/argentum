use serde::{Serialize, Serializer};
use std::error::Error;

#[derive(Debug)]
pub struct Conflict {
    pub source: Box<dyn Error>,
}

impl ToString for Conflict {
    fn to_string(&self) -> String {
        self.source.to_string()
    }
}

impl Conflict {
    pub fn new(source: Box<dyn Error>) -> Self {
        Self { source }
    }
}

impl Serialize for Conflict {
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
