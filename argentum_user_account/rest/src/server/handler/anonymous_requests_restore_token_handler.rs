use crate::dto::operation_response_enum::AnonymousRequestsRestoreTokenOperationResponseEnum;
use crate::dto::request::AnonymousRequestsRestoreTokenRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_user_business::entity::user::User;

use async_trait::async_trait;

#[async_trait]
pub trait AnonymousRequestsRestoreTokenTrait: Send + Sync {
    async fn handle(
        &self,
        _req: AnonymousRequestsRestoreTokenRequest,
        _user: User,
    ) -> Result<AnonymousRequestsRestoreTokenOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
