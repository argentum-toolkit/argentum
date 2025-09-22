use crate::entity::user::{AnonymousUser, UserTrait};
use crate::repository::user_repository::{AnonymousUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::error::PoisonedLockError;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

pub struct AnonymousUserRepositoryMock<I>
where
    I: Id,
{
    users: RwLock<HashMap<I, AnonymousUser<I>>>,
}

impl<I> AnonymousUserRepositoryMock<I>
where
    I: Id,
{
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl<I> Default for AnonymousUserRepositoryMock<I>
where
    I: Id,
{
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<I> AnonymousUserRepositoryTrait for AnonymousUserRepositoryMock<I>
where
    I: Id,
{
    type Id = I;

    async fn find(
        &self,
        id: Self::Id,
    ) -> Result<Option<AnonymousUser<Self::Id>>, ExternalUserError> {
        let guard = self
            .users
            .read()
            .map_err(|_| ExternalUserError::Anonymous(Some(PoisonedLockError::new_arc())))?;

        Ok(guard.get(&id).map(|u| AnonymousUser {
            id: u.id,
            created_at: u.created_at,
        }))
    }

    async fn save(&self, user: &AnonymousUser<Self::Id>) -> Result<(), ExternalUserError> {
        // TODO: check if key exists

        let u = AnonymousUser {
            id: user.id(),
            created_at: user.created_at,
        };

        match self
            .users
            .write()
            .map_err(|_| ExternalUserError::Anonymous(Some(Arc::new(PoisonedLockError))))?
            .insert(user.id(), u)
            .is_none()
        {
            true => Ok(()),
            false => Err(ExternalUserError::Anonymous(None)),
        }
    }
}
