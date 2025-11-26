use crate::data_type::Name;
use crate::entity::user::{AnonymousUser, AuthenticatedUser};
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::Id;

use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait AuthenticatedUserRepositoryTrait: Send + Sync {
    type Id: Id;

    async fn find(
        &self,
        id: Self::Id,
    ) -> Result<Option<AuthenticatedUser<Self::Id>>, ExternalUserError>;

    async fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<AuthenticatedUser<Self::Id>>, ExternalUserError>;

    // async fn create(&self, user: &AuthenticatedUser<Self::Id>) -> Result<(), ExternalUserError>;
    async fn create(
        &self,
        name: &Name,
        email: &EmailAddress,
    ) -> Result<AuthenticatedUser<Self::Id>, ExternalUserError>;
}

#[async_trait]
pub trait AnonymousUserRepositoryTrait: Send + Sync {
    type Id: Id;

    async fn find(
        &self,
        id: Self::Id,
    ) -> Result<Option<AnonymousUser<Self::Id>>, ExternalUserError>;
    async fn save(&self, user: &AnonymousUser<Self::Id>) -> Result<(), ExternalUserError>;
}

// Not business errors.
// This enum covers cases when errors where triggered via 3rd party libraries or via wrong configuration
#[derive(thiserror::Error, Debug)]
pub enum ExternalUserError {
    #[error("Can't save an user")]
    Authenticated(#[source] Option<Arc<dyn Error + Send + Sync>>),

    #[error("Can't save an anonymous")]
    Anonymous(#[source] Option<Arc<dyn Error + Send + Sync>>),
}
