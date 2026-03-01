use crate::entity::user::{Guest, UserTrait};
use crate::repository::user_repository::{ExternalUserError, GuestUserRepositoryTrait};
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::error::PoisonedLockError;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

pub struct GuestRepositoryMock<I>
where
    I: Id,
{
    users: RwLock<HashMap<I, Guest<I>>>,
}

impl<I> GuestRepositoryMock<I>
where
    I: Id,
{
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl<I> Default for GuestRepositoryMock<I>
where
    I: Id,
{
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<I> GuestUserRepositoryTrait for GuestRepositoryMock<I>
where
    I: Id,
{
    type Id = I;

    async fn find(&self, id: Self::Id) -> Result<Option<Guest<Self::Id>>, ExternalUserError> {
        let guard = self
            .users
            .read()
            .map_err(|_| ExternalUserError::Guest(Some(PoisonedLockError::new_arc())))?;

        Ok(guard.get(&id).map(|u| Guest {
            id: u.id,
            created_at: u.created_at,
        }))
    }

    async fn save(&self, user: &Guest<Self::Id>) -> Result<(), ExternalUserError> {
        // TODO: check if key exists

        let u = Guest {
            id: user.id(),
            created_at: user.created_at,
        };

        match self
            .users
            .write()
            .map_err(|_| ExternalUserError::Guest(Some(Arc::new(PoisonedLockError))))?
            .insert(user.id(), u)
            .is_none()
        {
            true => Ok(()),
            false => Err(ExternalUserError::Guest(None)),
        }
    }
}
