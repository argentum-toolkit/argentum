use crate::entity::credential::PasswordCredential;
use argentum_standard_business::data_type::id::Id;
use std::{error::Error, sync::Arc};

use async_trait::async_trait;

#[async_trait]
pub trait PasswordCredentialRepositoryTrait: Sync + Send {
    type Id: Id;

    async fn save(
        &self,
        cred: &PasswordCredential<Self::Id>,
    ) -> Result<(), PasswordCredentialRepositoryError>;

    async fn find_by_user_id(
        &self,
        id: Self::Id,
    ) -> Result<Option<PasswordCredential<Self::Id>>, PasswordCredentialRepositoryError>;

    async fn delete(
        &self,
        cred: &PasswordCredential<Self::Id>,
    ) -> Result<(), PasswordCredentialRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum PasswordCredentialRepositoryError {
    #[error("Can't save password credentials")]
    Save(#[source] Arc<dyn Error + Send + Sync>),

    #[error("Can't delete password credentials")]
    Delete(#[source] Arc<dyn Error + Send + Sync>),

    #[error("Can't find password credentials. Db error.")]
    Find(#[source] Arc<dyn Error + Send + Sync>),

    #[error("Password credentials error")]
    Other(#[source] Arc<dyn Error + Send + Sync>),
}
