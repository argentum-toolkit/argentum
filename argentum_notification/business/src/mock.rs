use crate::{Notification, NotificationError, NotificatorTrait};

use async_trait::async_trait;

pub struct StdoutNotificator {}

impl StdoutNotificator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StdoutNotificator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NotificatorTrait for StdoutNotificator {
    async fn send(&self, notification: Notification) -> Result<(), NotificationError> {
        println!(
            "---\nNOTIFICATION for user with id `{}`; \nSubject: `{}`\n`Body`:\n{}\n--- ",
            notification.user_id.to_string(),
            notification.subject,
            notification.body,
        );

        Ok(())
    }
}
