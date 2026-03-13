use crate::entity::user::{Guest, Member};
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::Id;

use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait MemberRepositoryTrait: Send + Sync {
    type Id: Id;

    async fn find(&self, id: Self::Id) -> Result<Option<Member<Self::Id>>, ExternalUserError>;

    async fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<Member<Self::Id>>, ExternalUserError>;

    async fn create(&self, member: &Member<Self::Id>) -> Result<(), ExternalUserError>;

    async fn update(&self, member: &Member<Self::Id>) -> Result<(), ExternalUserError>;
}

#[async_trait]
pub trait GuestUserRepositoryTrait: Send + Sync {
    type Id: Id;

    async fn find(&self, id: Self::Id) -> Result<Option<Guest<Self::Id>>, ExternalUserError>;
    async fn save(&self, user: &Guest<Self::Id>) -> Result<(), ExternalUserError>;
}

// Not business errors.
// This enum covers cases when errors where triggered via 3rd party libraries or via wrong configuration
#[derive(thiserror::Error, Debug, Clone)]
pub enum ExternalUserError {
    #[error("Can't save/load a member")]
    Member(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Can't save/load a guest")]
    Guest(#[source] Option<Arc<dyn Error + Send + Sync>>),
}
