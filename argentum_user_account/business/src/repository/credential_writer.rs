use crate::entity::credential::Credential;
use argentum_standard_business::data_type::id::Id;
use std::error::Error;

pub trait CredentialWriterTrait: Sync {
    fn write(&self, cred: Box<dyn Credential>) -> Result<(), CredentialWriterError>;

    fn delete_for_user(&self, user_id: &Id) -> Result<(), CredentialWriterError>;
}

#[derive(thiserror::Error, Debug)]
pub enum CredentialWriterError {
    #[error("Can't write credentials")]
    Credential(#[source] Box<dyn Error>),
}
