use crate::entity::EmailConfirmationToken;
use argentum_standard_business::data_type::id::Id;
use std::{error::Error, sync::Arc};

use async_trait::async_trait;

#[async_trait]
pub trait EmailConfirmationTokenRepositoryTrait: Sync + Send {
    type Id: Id;

    async fn find(
        &self,
        id: Self::Id,
    ) -> Result<Option<EmailConfirmationToken<Self::Id>>, EmailConfirmationTokenRepositoryError>;

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<EmailConfirmationToken<Self::Id>>, EmailConfirmationTokenRepositoryError>;

    async fn save(
        &self,
        token: &EmailConfirmationToken<Self::Id>,
    ) -> Result<(), EmailConfirmationTokenRepositoryError>;

    async fn delete_users_tokens(
        &self,
        user_id: Self::Id,
    ) -> Result<(), EmailConfirmationTokenRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum EmailConfirmationTokenRepositoryError {
    #[error("Can't save email confirmation token")]
    Save(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Can't delete email confirmation token")]
    Delete(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Can't find email confirmation token")]
    Find(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Email Confirmation Token repository error")]
    Other(#[source] Option<Arc<dyn Error + Send + Sync>>),
}
