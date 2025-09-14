use crate::entity::user::{AnonymousUser, UserTrait};
use crate::repository::user_repository::{AnonymousUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::id::Id;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

pub struct AnonymousUserRepositoryMock {
    users: RwLock<HashMap<Id, AnonymousUser>>,
}

impl AnonymousUserRepositoryMock {
    pub fn new() -> AnonymousUserRepositoryMock {
        AnonymousUserRepositoryMock {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for AnonymousUserRepositoryMock {
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
impl AnonymousUserRepositoryTrait for AnonymousUserRepositoryMock {
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

    async fn save(&self, user: &AnonymousUser) -> Result<(), ExternalUserError> {
        // TODO: check if key exists

        let u = AnonymousUser {
            id: user.id().clone(),
            created_at: user.created_at,
        };

        match self
            .users
            .write()
            .map_err(|_| ExternalUserError::Anonymous(Some(Arc::new(PoisonedLockError))))?
            .insert(user.id().clone(), u)
            .is_none()
        {
            true => Ok(()),
            false => Err(ExternalUserError::Anonymous(None)),
        }
    }
}
