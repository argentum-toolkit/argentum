use crate::entity::user::{AnonymousUser, AuthenticatedUser};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::Id;

use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait AuthenticatedUserRepositoryTrait: Send + Sync {
    async fn find(&self, id: &Id) -> Result<Option<AuthenticatedUser>, ExternalUserError>;
    async fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<AuthenticatedUser>, ExternalUserError>;
    async fn save(&self, user: &AuthenticatedUser) -> Result<(), ExternalUserError>;
}

#[async_trait]
pub trait AnonymousUserRepositoryTrait: Send + Sync {
    async fn find(&self, id: &Id) -> Result<Option<AnonymousUser>, ExternalUserError>;
    async fn save(&self, user: &AnonymousUser) -> Result<(), ExternalUserError>;
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
