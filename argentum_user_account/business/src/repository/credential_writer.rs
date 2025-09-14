use crate::entity::credential::Credential;
use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;
use std::{error::Error, sync::Arc};

#[async_trait]
pub trait CredentialWriterTrait: Sync {
    fn write(&self, cred: Box<dyn Credential>) -> CredentialWriterResult;

    fn delete_for_user(&self, user_id: &Id) -> CredentialWriterResult;
}

#[derive(thiserror::Error, Debug)]
pub enum CredentialWriterError {
    #[error("Can't write credentials")]
    Credential(#[source] Arc<dyn Error + Send + Sync>),
}

pub type CredentialWriterResult = Result<(), CredentialWriterError>;
