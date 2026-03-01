use crate::entity::user::Guest;
use crate::repository::user_repository::{ExternalUserError, GuestUserRepositoryTrait};
use argentum_standard_business::data_type::id::UIntId;
use argentum_standard_business::error::PoisonedLockError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub struct GuestRepositoryMockWithBrokenSave {
    users: RwLock<HashMap<UIntId, Guest<UIntId>>>,
}

impl GuestRepositoryMockWithBrokenSave {
    pub fn new() -> GuestRepositoryMockWithBrokenSave {
        GuestRepositoryMockWithBrokenSave {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for GuestRepositoryMockWithBrokenSave {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GuestUserRepositoryTrait for GuestRepositoryMockWithBrokenSave {
    type Id = UIntId;
    async fn find(&self, id: Self::Id) -> Result<Option<Guest<Self::Id>>, ExternalUserError> {
        let guard = self
            .users
            .read()
            .map_err(|_| ExternalUserError::Guest(Some(Arc::new(PoisonedLockError))))?;

        Ok(guard.get(&id).map(|u| Guest::<Self::Id> {
            id,
            created_at: u.created_at,
        }))
    }

    async fn save(&self, _user: &Guest<Self::Id>) -> Result<(), ExternalUserError> {
        Err(ExternalUserError::Guest(None))
    }
}
