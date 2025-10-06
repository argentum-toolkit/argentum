use std::fmt::Display;
use std::{error::Error, sync::Arc};

use serde::{Serialize, Serializer};

#[derive(Debug)]
pub struct Forbidden {
    pub source: Arc<dyn Error>,
}

impl Display for Forbidden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl Forbidden {
    pub fn new(source: Arc<dyn Error>) -> Self {
        Self { source }
    }
}

impl Serialize for Forbidden {
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

    use crate::data_type::error::Forbidden;

    #[test]
    fn test_serialize() -> Result<(), Box<dyn Error>> {
        let ie = Forbidden::new(Arc::new(ErrorMock {}));

        let actual = serde_json::to_string(&ie)?;

        assert_eq!("\"test error\"", actual);

        Ok(())
    }

    #[derive(thiserror::Error, Debug)]
    #[error("test error")]
    struct ErrorMock {}
}
