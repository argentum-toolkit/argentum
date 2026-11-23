use crate::entity::Message;
use argentum_standard_business::data_type::id::Id;
use std::{error::Error, sync::Arc};

use async_trait::async_trait;

#[async_trait]
pub trait MessageRepositoryTrait: Sync + Send {
    type Id: Id;

    async fn find(&self, id: Self::Id)
    -> Result<Option<Message<Self::Id>>, MessageRepositoryError>;

    async fn save(&self, message: &Message<Self::Id>) -> Result<(), MessageRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum MessageRepositoryError {
    #[error("Can't save message")]
    Save(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Can't find message")]
    Find(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Message repository error")]
    Other(#[source] Option<Arc<dyn Error + Send + Sync>>),
}
