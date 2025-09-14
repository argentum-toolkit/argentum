use crate::dto::operation_response_enum::UserLoginsWithPasswordOperationResponseEnum;
use crate::dto::request::UserLoginsWithPasswordRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_user_business::entity::user::User;

use async_trait::async_trait;

#[async_trait]
pub trait UserLoginsWithPasswordTrait: Send + Sync {
    async fn handle(
        &self,
        _req: UserLoginsWithPasswordRequest,
        _user: User,
    ) -> Result<UserLoginsWithPasswordOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
