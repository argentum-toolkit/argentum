use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryError;
use crate::repository::user_repository::SavingUserError;
use argentum_encryption_business::password::EncryptionError;

#[derive(thiserror::Error, Debug)]
pub enum RestorePasswordError {
    #[error("Can't get user data. DB error")]
    GetUserError(#[from] SavingUserError),

    #[error("User is not found")]
    UserNotFoundError,

    #[error("Token is not found")]
    TokenNotFoundError,

    #[error("Token expired")]
    TokenExpired,

    #[error("Can't save token")]
    TokenRepositoryError(#[from] RestorePasswordTokenRepositoryError),

    #[error("Can't encrypt new password")]
    PasswordEncryptionError(#[from] EncryptionError),
}
