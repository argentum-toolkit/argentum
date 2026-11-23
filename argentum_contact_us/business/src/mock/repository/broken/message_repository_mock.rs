use crate::entity::Message;
use crate::repository::message_repository::MessageRepositoryError;
use crate::repository::message_repository::MessageRepositoryTrait;
use argentum_standard_business::data_type::id::UIntId;
use argentum_standard_business::error::PoisonedLockError;
use async_trait::async_trait;
use std::sync::Arc;

pub struct MessageRepositoryMockWithBrokenSave;

impl MessageRepositoryMockWithBrokenSave {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MessageRepositoryMockWithBrokenSave {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MessageRepositoryTrait for MessageRepositoryMockWithBrokenSave {
    type Id = UIntId;

    async fn find(
        &self,
        _id: Self::Id,
    ) -> Result<Option<Message<Self::Id>>, MessageRepositoryError> {
        Err(MessageRepositoryError::Find(Some(Arc::new(
            PoisonedLockError,
        ))))
    }

    async fn save(&self, _message: &Message<Self::Id>) -> Result<(), MessageRepositoryError> {
        Err(MessageRepositoryError::Save(Some(Arc::new(
            PoisonedLockError,
        ))))
    }
}
