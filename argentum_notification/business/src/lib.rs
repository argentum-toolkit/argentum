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

pub struct Notification<I>
where
    I: Id,
{
    pub user_id: I,
    pub body: String,
    pub subject: String,
}

impl<I> Notification<I>
where
    I: Id,
{
    pub fn new(user_id: I, body: String, subject: String) -> Self {
        Self {
            user_id,
            body,
            subject,
        }
    }
}

#[cfg(feature = "server")]
#[async_trait]
pub trait NotifierTrait: Sync + Send {
    type Id: Id;

    async fn send(&self, notification: Notification<Self::Id>) -> Result<(), NotificationError>;
}
