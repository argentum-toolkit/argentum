use std::sync::Arc;

use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::{
    entity::user::AuthenticatedUser,
    repository::{
        session_repository::SessionRepositoryTrait,
        user_repository::AuthenticatedUserRepositoryTrait,
    },
    service::{MemberAuthenticatorError, MemberAuthenticatorTrait},
};
use async_trait::async_trait;

pub struct MemberAuthenticator {
    member_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = UniqueId>>,
    session_repository: Arc<dyn SessionRepositoryTrait<Id = UniqueId>>,
}

impl MemberAuthenticator {
    pub fn new(
        member_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = UniqueId>>,
        session_repository: Arc<dyn SessionRepositoryTrait<Id = UniqueId>>,
    ) -> Self {
        Self {
            member_repository,
            session_repository,
        }
    }
}

#[async_trait]
impl MemberAuthenticatorTrait for MemberAuthenticator {
    type Id = UniqueId;

    async fn authenticate(
        &self,
        token: &str,
    ) -> Result<Option<AuthenticatedUser<Self::Id>>, MemberAuthenticatorError> {
        let session = match self.session_repository.find_by_token(token).await {
            Ok(Some(s)) => s,
            Ok(None) => return Err(MemberAuthenticatorError::Authenticate(None)),
            Err(e) => return Err(MemberAuthenticatorError::Other(Some(Arc::new(e)))),
        };

        match self.member_repository.find(session.user_id).await {
            Ok(Some(u)) => Ok(Some(u)),
            Ok(None) => Ok(None),
            Err(e) => return Err(MemberAuthenticatorError::Other(Some(Arc::new(e)))),
        }
    }
}
