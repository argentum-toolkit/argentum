use crate::dto::request::UserRegistersWithPasswordRequest;
use crate::dto::operation_response_enum::UserRegistersWithPasswordOperationResponseEnum;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::entity::user::User;

use async_trait::async_trait;

#[async_trait]
pub trait UserRegistersWithPasswordTrait: Send + Sync {
    type Id: Id;

    async fn handle(
        &self,
        _req: UserRegistersWithPasswordRequest,
        _user: User<Self::Id>,
    ) -> Result<UserRegistersWithPasswordOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
