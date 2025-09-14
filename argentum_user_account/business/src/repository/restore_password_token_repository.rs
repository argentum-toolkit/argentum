use crate::entity::restore_password_token::RestorePasswordToken;
use argentum_standard_business::data_type::id::Id;
use std::{error::Error, sync::Arc};

use async_trait::async_trait;

#[async_trait]
pub trait RestorePasswordTokenRepositoryTrait: Sync + Send {
    async fn find(
        &self,
        id: &Id,
    ) -> Result<Option<RestorePasswordToken>, RestorePasswordTokenRepositoryError>;

    async fn find_by_token(
        &self,
        token: String,
    ) -> Result<Option<RestorePasswordToken>, RestorePasswordTokenRepositoryError>;

    async fn save(
        &self,
        token: &RestorePasswordToken,
    ) -> Result<(), RestorePasswordTokenRepositoryError>;

    async fn delete_users_tokens(
        &self,
        user_id: &Id,
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
