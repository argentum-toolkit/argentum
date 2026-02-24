use crate::entity::user::{Member, UserTrait};
use crate::repository::user_repository::{ExternalUserError, MemberRepositoryTrait};
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::UIntId;
use argentum_standard_business::error::PoisonedLockError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub struct MemberRepositoryMockWihBrokenSave {
    users: RwLock<HashMap<UIntId, Member<UIntId>>>,
}

impl MemberRepositoryMockWihBrokenSave {
    pub fn new() -> MemberRepositoryMockWihBrokenSave {
        MemberRepositoryMockWihBrokenSave {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemberRepositoryMockWihBrokenSave {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MemberRepositoryTrait for MemberRepositoryMockWihBrokenSave {
    type Id = UIntId;

    async fn find(&self, id: Self::Id) -> Result<Option<Member<Self::Id>>, ExternalUserError> {
        Ok(self
            .users
            .read()
            .map_err(|_| ExternalUserError::Member(Some(Arc::new(PoisonedLockError))))?
            .get(&id)
            .map(|u| Member::<Self::Id>::new(u.id(), u.name.clone(), u.email.clone())))
    }

    async fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<Member<Self::Id>>, ExternalUserError> {
        let guard = self
            .users
            .read()
            .map_err(|_| ExternalUserError::Guest(Some(Arc::new(PoisonedLockError))))?;

        for (_, u) in guard.iter() {
            if &u.email == email {
                return Ok(Some(Member::<Self::Id>::new(
                    u.id(),
                    u.name.clone(),
                    u.email.clone(),
                )));
            }
        }

        Ok(None)
    }

    async fn create(&self, _user: &Member<Self::Id>) -> Result<(), ExternalUserError> {
        Err(ExternalUserError::Member(None))
    }
}
