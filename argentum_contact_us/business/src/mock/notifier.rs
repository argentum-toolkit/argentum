use crate::entity::Message;
use crate::notifier::admin_notifier::{AdminNotifierError, AdminNotifierTrait};
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::error::PoisonedLockError;
use async_trait::async_trait;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct AdminNotifierMock<I>
where
    I: Id,
{
    sent: RwLock<Vec<Message<I>>>,
    answer: Option<()>,
}

impl<I> AdminNotifierMock<I>
where
    I: Id,
{
    pub fn new() -> Self {
        Self::passing()
    }

    pub fn passing() -> Self {
        Self {
            sent: RwLock::new(Vec::new()),
            answer: Some(()),
        }
    }

    pub fn failing() -> Self {
        Self {
            sent: RwLock::new(Vec::new()),
            answer: None,
        }
    }

    pub fn sent(&self) -> Vec<Message<I>> {
        self.sent
            .read()
            .expect("RwLock should not be poisoned")
            .clone()
    }
}

#[async_trait]
impl<I> AdminNotifierTrait for AdminNotifierMock<I>
where
    I: Id,
{
    type Id = I;

    async fn send(&self, message: &Message<Self::Id>) -> Result<(), AdminNotifierError> {
        self.sent
            .write()
            .expect("RwLock should not be poisoned")
            .push(message.clone());

        match self.answer {
            Some(()) => Ok(()),
            None => Err(AdminNotifierError::External(Arc::new(PoisonedLockError))),
        }
    }
}
