use std::{error::Error, sync::Arc};

use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;

use crate::entity::Credential;

#[async_trait]
pub trait CredentialRepositoryTrait: Send + Sync {
    type UserId: Id;

    async fn insert(
        &self,
        credential: &Credential<Self::UserId>,
    ) -> Result<(), CredentialRepositoryError>;

    async fn find_by_external_id(
        &self,
        external_id: &str,
        provider: &str,
    ) -> Result<Option<Credential<Self::UserId>>, CredentialRepositoryError>;
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum CredentialRepositoryError {
    #[error("Can't save OIDC credential")]
    Save(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Can't find OIDC credential")]
    Find(#[source] Option<Arc<dyn Error + Send + Sync>>),
}
