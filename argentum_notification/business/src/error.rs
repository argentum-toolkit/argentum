use argentum_user_business::repository::user_repository::ExternalUserError;

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
