use std::{error::Error, sync::Arc};

#[derive(Debug)]
pub struct PoisonedLockError;

impl PoisonedLockError {
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl std::fmt::Display for PoisonedLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "`RwLock` is poisoned")
    }
}

impl Error for PoisonedLockError {}
