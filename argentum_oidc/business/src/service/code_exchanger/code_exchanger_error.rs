use std::{error::Error, sync::Arc};

use crate::service::provider::ProviderError;

#[derive(thiserror::Error, Debug)]
pub enum CodeExchangerError {
    #[error("Provider error: `{0}`")]
    Provider(
        #[source]
        #[from]
        ProviderError,
    ),

    #[error("Cant provide token data. Error: `{0}`")]
    TokenData(String),

    #[error("Can't exchange code. Network error")]
    Network(String),

    #[error("Can't exchange code. Bad response status: `{0}`")]
    OidcBadStatus(u16),

    #[error("OIDC provider not found. Provider name : `{0}`")]
    OidcProviderNotFound(String),

    #[error("Other token exchange error")]
    Other(
        #[source]
        #[from]
        Option<Arc<dyn Error + Send + Sync>>,
    ),
}
