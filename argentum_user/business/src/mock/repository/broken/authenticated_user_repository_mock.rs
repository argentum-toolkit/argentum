use crate::entity::user::{AuthenticatedUser, UserTrait};
use crate::repository::user_repository::{AuthenticatedUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;

pub struct AuthenticatedUserRepositoryMockWihBrokenSave {
    users: RwLock<HashMap<Id, AuthenticatedUser>>,
}

impl AuthenticatedUserRepositoryMockWihBrokenSave {
    pub fn new() -> AuthenticatedUserRepositoryMockWihBrokenSave {
        AuthenticatedUserRepositoryMockWihBrokenSave {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for AuthenticatedUserRepositoryMockWihBrokenSave {
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
impl AuthenticatedUserRepositoryTrait for AuthenticatedUserRepositoryMockWihBrokenSave {
    async fn find(&self, id: &Id) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        Ok(self
            .users
            .read()
            .map_err(|_| ExternalUserError::Authenticated(Some(Arc::new(PoisonedLockError))))?
            .get(id)
            .map(|u| AuthenticatedUser::new(&u.id().clone(), u.name.clone(), u.email.clone())))
    }

    async fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        let guard = self
            .users
            .read()
            .map_err(|_| ExternalUserError::Anonymous(Some(Arc::new(PoisonedLockError))))?;

        for (_, u) in guard.iter() {
            if &u.email == email {
                return Ok(Some(AuthenticatedUser::new(
                    &u.id().clone(),
                    u.name.clone(),
                    u.email.clone(),
                )));
            }
        }

        Ok(None)
    }

    async fn save(&self, _user: &AuthenticatedUser) -> Result<(), ExternalUserError> {
        Err(ExternalUserError::Authenticated(None))
    }
}
