//! Notification component
//!
//! The abstract way to send notification to user by id
//! Provides a macro to create and manage your own events
//!
pub mod mock;

use argentum_standard_business::data_type::id::Id;
use argentum_user_business::repository::user_repository::ExternalUserError;

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

#[derive(thiserror::Error, Debug)]
pub enum NotificationError {
    #[error("Notification can't be sent. User storage error")]
    UserRepositoryError(#[from] ExternalUserError),

    #[error("Notification can't be sent. User Not Found")]
    UserNotFoundError,

    #[error("Notification can't be sent. External error")]
    MessageSendingExternalError,
}

pub trait NotificatorTrait: Sync + Send {
    fn send(&self, notification: Notification) -> Result<(), NotificationError>;
}
