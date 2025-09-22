use crate::entity::user::{AuthenticatedUser, UserTrait};
use crate::repository::user_repository::{AuthenticatedUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::error::PoisonedLockError;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

#[derive(Default)]
pub struct AuthenticatedUserRepositoryMock<I>
where
    I: Id,
{
    users: RwLock<HashMap<I, AuthenticatedUser<I>>>,
}

impl<I> AuthenticatedUserRepositoryMock<I>
where
    I: Id,
{
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl<I> AuthenticatedUserRepositoryTrait for AuthenticatedUserRepositoryMock<I>
where
    I: Id,
{
    type Id = I;

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
            .map_err(|_| ExternalUserError::Authenticated(Some(Arc::new(PoisonedLockError))))?;

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

    async fn save(&self, user: &AuthenticatedUser<Self::Id>) -> Result<(), ExternalUserError> {
        // TODO: check if key exists. Eg:
        // if self.users. contains_key(user.get_id().clone()) {
        //     return Err("Already exists");
        // }

        let u = AuthenticatedUser::<Self::Id> {
            id: user.id(),
            created_at: user.created_at,
            name: user.name.clone(),
            email: user.email.clone(),
        };

        match self
            .users
            .write()
            .map_err(|_| ExternalUserError::Authenticated(Some(Arc::new(PoisonedLockError))))?
            .insert(user.id(), u)
            .is_none()
        {
            true => Ok(()),
            false => Err(ExternalUserError::Authenticated(None)),
        }
    }
}
