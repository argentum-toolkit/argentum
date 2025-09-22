use crate::entity::restore_password_token::RestorePasswordToken;
use argentum_standard_business::data_type::id::Id;
use std::{error::Error, sync::Arc};

use async_trait::async_trait;

#[async_trait]
pub trait RestorePasswordTokenRepositoryTrait: Sync + Send {
    type Id: Id;

    async fn find(
        &self,
        id: Self::Id,
    ) -> Result<Option<RestorePasswordToken<Self::Id>>, RestorePasswordTokenRepositoryError>;

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<RestorePasswordToken<Self::Id>>, RestorePasswordTokenRepositoryError>;

    async fn save(
        &self,
        token: &RestorePasswordToken<Self::Id>,
    ) -> Result<(), RestorePasswordTokenRepositoryError>;

    async fn delete_users_tokens(
        &self,
        user_id: Self::Id,
    ) -> Result<(), RestorePasswordTokenRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum RestorePasswordTokenRepositoryError {
    #[error("Can't save restore password token")]
    Save(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Can't delete restore password token")]
    Delete(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Can't find restore password token")]
    Find(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Restore Password Token repository error")]
    Other(#[source] Option<Arc<dyn Error + Send + Sync>>),
}
