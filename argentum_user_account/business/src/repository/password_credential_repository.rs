use crate::entity::credential::PasswordCredential;
use argentum_standard_business::data_type::id::Id;
use std::error::Error;

pub trait PasswordCredentialRepositoryTrait: Sync + Send {
    fn save(&self, cred: &PasswordCredential) -> Result<(), PasswordCredentialRepositoryError>;

    fn find_by_user_id(
        &self,
        id: &Id,
    ) -> Result<Option<PasswordCredential>, PasswordCredentialRepositoryError>;

    fn delete(&self, cred: &PasswordCredential) -> Result<(), PasswordCredentialRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum PasswordCredentialRepositoryError {
    #[error("Can't save password credentials")]
    Save(#[source] Box<dyn Error>),

    #[error("Can't delete password credentials")]
    Delete(#[source] Box<dyn Error>),

    #[error("Can't fidn password credentials")]
    Find(#[source] Box<dyn Error>),

    #[error("Password Credentials repository error")]
    Other(#[source] Option<Box<dyn Error>>),
}
