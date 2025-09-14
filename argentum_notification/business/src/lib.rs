//! Notification component
//!
//! The abstract way to send notification to user by id
//! Provides a macro to create and manage your own events
//!

#[cfg(feature = "server")]
mod error;

#[cfg(feature = "server")]
pub use error::NotificationError;

#[cfg(feature = "server")]
pub mod mock;

use argentum_standard_business::data_type::id::Id;

#[cfg(feature = "server")]
use async_trait::async_trait;

pub struct Notification {
    pub user_id: Id,
    pub body: String,
    pub subject: String,
}

impl Notification {
    pub fn new(user_id: Id, body: String, subject: String) -> Notification {
        Notification {
            user_id,
            body,
            subject,
        }
    }
}

#[cfg(feature = "server")]
#[async_trait]
pub trait NotificatorTrait: Sync + Send {
    async fn send(&self, notification: Notification) -> Result<(), NotificationError>;
}
