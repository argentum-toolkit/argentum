use crate::entity::Message;
use crate::repository::message_repository::{MessageRepositoryError, MessageRepositoryTrait};
use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;
use std::sync::RwLock;

#[derive(Default)]
pub struct MessageRepositoryMock<I>
where
    I: Id,
{
    messages: RwLock<Vec<Message<I>>>,
}

impl<I> MessageRepositoryMock<I>
where
    I: Id,
{
    pub fn new() -> Self {
        Self {
            messages: RwLock::new(Vec::new()),
        }
    }

    pub fn messages(&self) -> Vec<Message<I>> {
        self.messages
            .read()
            .expect("RwLock should not be poisoned")
            .clone()
    }

    pub fn len(&self) -> usize {
        self.messages
            .read()
            .expect("RwLock should not be poisoned")
            .len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[async_trait]
impl<I> MessageRepositoryTrait for MessageRepositoryMock<I>
where
    I: Id,
{
    type Id = I;

    async fn find(
        &self,
        id: Self::Id,
    ) -> Result<Option<Message<Self::Id>>, MessageRepositoryError> {
        Ok(self
            .messages
            .read()
            .expect("RwLock should not be poisoned")
            .iter()
            .find(|message| message.id == id)
            .cloned())
    }

    async fn save(&self, message: &Message<Self::Id>) -> Result<(), MessageRepositoryError> {
        self.messages
            .write()
            .expect("RwLock should not be poisoned")
            .push(message.clone());

        Ok(())
    }
}
