use argentum_standard_business::error::PoisonedLockError;

#[derive(thiserror::Error, Debug)]
pub enum AdminApiError {
    #[error("Network error")]
    Network(#[from] reqwest::Error),

    #[error("Admin API error")]
    Other(#[from] PoisonedLockError),
}
