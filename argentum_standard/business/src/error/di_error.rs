use std::sync::{MutexGuard, PoisonError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiError {
    #[error("Mutex was poisoned")]
    MutexPoisoned,

    //TODO: remove String error
    #[error("{0}")]
    String(String),

    #[error("Other Error: {0}")]
    Other(String),
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for DiError {
    fn from(_: PoisonError<MutexGuard<'_, T>>) -> Self {
        DiError::MutexPoisoned
    }
}

impl From<String> for DiError {
    fn from(err: String) -> Self {
        DiError::String(err)
    }
}

impl From<&str> for DiError {
    fn from(err: &str) -> Self {
        DiError::String(err.to_string())
    }
}
