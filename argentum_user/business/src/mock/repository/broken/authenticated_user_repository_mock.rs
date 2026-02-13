use crate::entity::user::{AuthenticatedUser, UserTrait};
use crate::repository::user_repository::{AuthenticatedUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::UIntId;
use argentum_standard_business::error::PoisonedLockError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub struct AuthenticatedUserRepositoryMockWihBrokenSave {
    users: RwLock<HashMap<UIntId, AuthenticatedUser<UIntId>>>,
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

#[async_trait]
impl AuthenticatedUserRepositoryTrait for AuthenticatedUserRepositoryMockWihBrokenSave {
    type Id = UIntId;

    async fn find(
        &self,
        id: Self::Id,
    ) -> Result<Option<AuthenticatedUser<Self::Id>>, ExternalUserError> {
        Ok(self
            .users
            .read()
            .map_err(|_| ExternalUserError::Authenticated(Some(Arc::new(PoisonedLockError))))?
            .get(&id)
            .map(|u| AuthenticatedUser::<Self::Id>::new(u.id(), u.name.clone(), u.email.clone())))
    }

    async fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<AuthenticatedUser<Self::Id>>, ExternalUserError> {
        let guard = self
            .users
            .read()
            .map_err(|_| ExternalUserError::Anonymous(Some(Arc::new(PoisonedLockError))))?;

        for (_, u) in guard.iter() {
            if &u.email == email {
                return Ok(Some(AuthenticatedUser::<Self::Id>::new(
                    u.id(),
                    u.name.clone(),
                    u.email.clone(),
                )));
            }
        }

        Ok(None)
    }

    async fn create(&self, _user: &AuthenticatedUser<Self::Id>) -> Result<(), ExternalUserError> {
        Err(ExternalUserError::Authenticated(None))
    }
}
