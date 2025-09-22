use crate::entity::session::Session;

use async_trait::async_trait;

use argentum_standard_business::data_type::id::Id;
use std::{error::Error, sync::Arc};

#[async_trait]
pub trait SessionRepositoryTrait: Send + Sync {
    type Id: Id;

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<Session<Self::Id>>, SessionRepositoryError>;
    async fn save(&self, session: &Session<Self::Id>) -> Result<(), SessionRepositoryError>;
    async fn delete_users_sessions(&self, user_id: Self::Id) -> Result<(), SessionRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum SessionRepositoryError {
    #[error("Can't save session")]
    Save(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Can't delete session")]
    Delete(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Session repository error")]
    Other(#[source] Option<Arc<dyn Error + Send + Sync>>),
}
