use crate::entity::user::AnonymousUser;
use crate::repository::user_repository::{AnonymousUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;

pub struct AnonymousRepositoryMockWithBrokenSave {
    users: RwLock<HashMap<Id, AnonymousUser>>,
}

impl AnonymousRepositoryMockWithBrokenSave {
    pub fn new() -> AnonymousRepositoryMockWithBrokenSave {
        AnonymousRepositoryMockWithBrokenSave {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for AnonymousRepositoryMockWithBrokenSave {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct PoisonedLockError;

impl std::fmt::Display for PoisonedLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "`RwLock` is poisoned")
    }
}

impl Error for PoisonedLockError {}

#[async_trait]
impl AnonymousUserRepositoryTrait for AnonymousRepositoryMockWithBrokenSave {
    async fn find(&self, id: &Id) -> Result<Option<AnonymousUser>, ExternalUserError> {
        let guard = self
            .users
            .read()
            .map_err(|_| ExternalUserError::Anonymous(Some(Arc::new(PoisonedLockError))))?;

        Ok(guard.get(id).map(|u| AnonymousUser {
            id: u.id.clone(),
            created_at: u.created_at,
        }))
    }

    async fn save(&self, _user: &AnonymousUser) -> Result<(), ExternalUserError> {
        Err(ExternalUserError::Anonymous(None))
    }
}
