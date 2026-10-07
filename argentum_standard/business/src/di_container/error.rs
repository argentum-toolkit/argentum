use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiError {
    #[error("DI service `{type_name}` is not registered")]
    NotRegistered { type_name: &'static str },

    #[error("DI service `{type_name}` has wrong type")]
    TypeMismatch { type_name: &'static str },

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl DiError {
    pub fn with_context<E>(message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Other(anyhow::Error::new(source).context(message.into()))
    }
}

impl From<String> for DiError {
    fn from(message: String) -> Self {
        Self::Other(anyhow::Error::msg(message))
    }
}

impl From<&str> for DiError {
    fn from(message: &str) -> Self {
        Self::Other(anyhow::Error::msg(message.to_owned()))
    }
}
