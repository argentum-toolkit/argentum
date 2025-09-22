use std::{error::Error, sync::Arc};

#[derive(thiserror::Error, Debug)]
pub enum CredentialWriterError {
    #[error("Can't write credentials")]
    Credential(#[source] Arc<dyn Error + Send + Sync>),
}

pub type CredentialWriterResult = Result<(), CredentialWriterError>;
