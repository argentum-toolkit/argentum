use crate::{Notification, NotificationError, NotificatorTrait};

use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;

#[derive(Default)]
pub struct StdoutNotificator<I>
where
    I: Id,
{
    _phantom: std::marker::PhantomData<I>,
}

impl<I> StdoutNotificator<I>
where
    I: Id,
{
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<I> NotificatorTrait for StdoutNotificator<I>
where
    I: Id,
{
    type Id = I;
    async fn send(&self, notification: Notification<Self::Id>) -> Result<(), NotificationError> {
        println!(
            "---\nNOTIFICATION for user with id `{}`; \nSubject: `{}`\n`Body`:\n{}\n--- ",
            notification.user_id, notification.subject, notification.body,
        );

        Ok(())
    }
}
