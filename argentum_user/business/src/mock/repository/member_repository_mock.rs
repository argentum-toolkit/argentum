use crate::entity::user::{Member, UserTrait};
use crate::repository::user_repository::{ExternalUserError, MemberRepositoryTrait};
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::error::PoisonedLockError;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

#[derive(Default)]
pub struct MemberRepositoryMock<I>
where
    I: Id,
{
    users: RwLock<HashMap<I, Member<I>>>,
}

impl<I> MemberRepositoryMock<I>
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
impl<I> MemberRepositoryTrait for MemberRepositoryMock<I>
where
    I: Id,
{
    type Id = I;

    async fn find(&self, id: Self::Id) -> Result<Option<Member<Self::Id>>, ExternalUserError> {
        Ok(self
            .users
            .read()
            .map_err(|_| ExternalUserError::Member(Some(Arc::new(PoisonedLockError))))?
            .get(&id)
            .map(|u| {
                Member::<Self::Id>::new(u.id(), u.name.clone(), u.email.clone(), u.email_confirmed)
            }))
    }

    async fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<Member<Self::Id>>, ExternalUserError> {
        let guard = self
            .users
            .read()
            .map_err(|_| ExternalUserError::Member(Some(Arc::new(PoisonedLockError))))?;

        for u in guard.values() {
            if &u.email == email {
                return Ok(Some(Member::<Self::Id>::new(
                    u.id(),
                    u.name.clone(),
                    u.email.clone(),
                    u.email_confirmed,
                )));
            }
        }

        Ok(None)
    }

    async fn create(&self, user: &Member<Self::Id>) -> Result<(), ExternalUserError> {
        // TODO: check if key exists. Eg:
        // if self.users. contains_key(user.get_id().clone()) {
        //     return Err("Already exists");
        // }

        match self
            .users
            .write()
            .map_err(|_| ExternalUserError::Member(Some(Arc::new(PoisonedLockError))))?
            .insert(user.id, user.clone())
            .is_none()
        {
            true => Ok(()),
            false => Err(ExternalUserError::Member(None)),
        }
    }

    async fn update(&self, user: &Member<Self::Id>) -> Result<(), ExternalUserError> {
        let mut guard = self
            .users
            .write()
            .map_err(|_| ExternalUserError::Member(Some(Arc::new(PoisonedLockError))))?;

        if let Some(existing_user) = guard.get_mut(&user.id) {
            *existing_user = user.clone();
            Ok(())
        } else {
            Err(ExternalUserError::Member(None))
        }
    }
}
