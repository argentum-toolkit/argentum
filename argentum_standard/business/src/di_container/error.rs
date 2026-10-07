use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiError {
    #[error("DI service `{type_name}` is not registered")]
    NotRegistered { type_name: &'static str },

    #[error("DI service `{type_name}` has wrong type")]
    TypeMismatch { type_name: &'static str },

    #[error("{0}")]
    Other(String),
}

impl From<String> for DiError {
    fn from(message: String) -> Self {
        Self::Other(message)
    }
}

impl From<&str> for DiError {
    fn from(message: &str) -> Self {
        Self::Other(message.to_owned())
    }
}
