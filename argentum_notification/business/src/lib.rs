//! Notification component
//!
//! The abstract way to send notification to user by id
//! Provides a macro to create and manage your own events
//!
pub mod mock;

use argentum_standard_business::data_type::id::Id;
use argentum_user_business::repository::user_repository::ExternalUserError;

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

#[derive(thiserror::Error, Debug)]
pub enum NotificationError {
    #[error("Notification can't be sent. User storage error")]
    UserRepositoryError(#[from] ExternalUserError),

    #[error("Notification can't be sent. User Not Found")]
    UserNotFoundError,

    #[error("Notification can't be sent. Wrong `from` address `{0}`. Error: `{1}`")]
    FromError(String, String),

    #[error("Notification can't be sent. Wrong `to` address `{0}`. Error: `{1}`")]
    ToError(String, String),

    #[error("Notification can't be sent. External error")]
    MessageSendingExternalError,
}

#[async_trait]
pub trait NotificatorTrait: Sync + Send {
    async fn send(&self, notification: Notification) -> Result<(), NotificationError>;
}
