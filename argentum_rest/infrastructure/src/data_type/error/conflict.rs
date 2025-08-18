use serde::{Serialize, Serializer};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Conflict {
    pub source: Box<dyn Error>,
}

impl Display for Conflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
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
    use std::error::Error;

    use crate::data_type::error::Conflict;

    #[test]
    fn test_serialize() -> Result<(), Box<dyn Error>> {
        let ie = Conflict::new(Box::new(ErrorMock {}));

        let actual = serde_json::to_string(&ie)?;

        assert_eq!("\"test error\"", actual);

        Ok(())
    }

    #[derive(thiserror::Error, Debug)]
    #[error("test error")]
    struct ErrorMock {}
}
