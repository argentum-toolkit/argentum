use std::{error::Error, sync::Arc};

#[derive(thiserror::Error, Debug, Clone)]
pub enum ProviderError {
    #[error("Network error")]
    Network(#[source] Arc<dyn Error + Send + Sync>),

    #[error("Can't exchange code. Bad response status: `{0}`")]
    BadStatus(u16),

    #[error("Cant provide token data. Error: `{0}`")]
    TokenData(String),
}
