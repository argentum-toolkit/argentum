use crate::dto::operation_response_enum::UserRegistersWithPasswordOperationResponseEnum;
use crate::dto::request::UserRegistersWithPasswordRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_user_business::entity::user::User;

use async_trait::async_trait;

#[async_trait]
pub trait UserRegistersWithPasswordTrait: Send + Sync {
    async fn handle(
        &self,
        _req: UserRegistersWithPasswordRequest,
        _user: User,
    ) -> Result<UserRegistersWithPasswordOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
