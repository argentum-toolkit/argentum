use std::{error::Error, sync::Arc};

#[derive(thiserror::Error, Debug, Clone)]
pub enum ProviderError {
    #[error("Network error")]
    Network(#[source] Arc<dyn Error + Send + Sync>),

    #[error("Can't exchange code. Bad response status: `{0}`. Response: {1}")]
    BadStatus(u16, String),

    #[error("Cant provide token data. Error: `{0}`")]
    TokenData(String),
}
