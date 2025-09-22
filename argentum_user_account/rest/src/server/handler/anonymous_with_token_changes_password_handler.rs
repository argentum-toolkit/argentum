use crate::dto::operation_response_enum::AnonymousWithTokenChangesPasswordOperationResponseEnum;
use crate::dto::request::AnonymousWithTokenChangesPasswordRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::entity::user::User;

use async_trait::async_trait;

#[async_trait]
pub trait AnonymousWithTokenChangesPasswordTrait: Send + Sync {
    type Id: Id;

    async fn handle(
        &self,
        _req: AnonymousWithTokenChangesPasswordRequest,
        _user: User<Self::Id>,
    ) -> Result<AnonymousWithTokenChangesPasswordOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
