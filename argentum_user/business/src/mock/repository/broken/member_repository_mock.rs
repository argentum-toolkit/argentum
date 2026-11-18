use crate::entity::user::Member;
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
            .cloned())
    }

    async fn find_by_slug(
        &self,
        slug: &str,
    ) -> Result<Option<Member<Self::Id>>, ExternalUserError> {
        Ok(self
            .users
            .read()
            .map_err(|_| ExternalUserError::Member(Some(Arc::new(PoisonedLockError))))?
            .values()
            .find(|member| member.slug == slug)
            .cloned())
    }

    async fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<Member<Self::Id>>, ExternalUserError> {
        let guard = self
            .users
            .read()
            .map_err(|_| ExternalUserError::Guest(Some(Arc::new(PoisonedLockError))))?;

        Ok(guard
            .values()
            .find(|member| &member.email == email)
            .cloned())
    }

    async fn create(&self, _user: &Member<Self::Id>) -> Result<(), ExternalUserError> {
        Err(ExternalUserError::Member(None))
    }

    async fn update(&self, _user: &Member<Self::Id>) -> Result<(), ExternalUserError> {
        Err(ExternalUserError::Member(None))
    }

    async fn delete(&self, _id: Self::Id) -> Result<(), ExternalUserError> {
        Err(ExternalUserError::Member(None))
    }
}
