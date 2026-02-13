use argentum_standard_business::data_type::id::Id;

use crate::entity::user::AuthenticatedUser;
use std::{error::Error, sync::Arc};

use async_trait::async_trait;

#[async_trait]
pub trait MemberAuthenticatorTrait: Send + Sync {
    type Id: Id;

    async fn authenticate(
        &self,
        token: &str,
    ) -> Result<Option<AuthenticatedUser<Self::Id>>, MemberAuthenticatorError>;
}

#[derive(thiserror::Error, Debug)]
pub enum MemberAuthenticatorError {
    #[error("Can't authenticate a member")]
    Authenticate(#[source] Option<Box<dyn Error + Send + Sync>>),

    #[error("Member authenticator error")]
    Other(#[source] Option<Arc<dyn Error + Send + Sync>>),
}
