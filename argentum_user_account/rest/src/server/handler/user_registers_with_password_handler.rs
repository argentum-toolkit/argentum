use crate::dto::operation_response_enum::UserRegistersWithPasswordOperationResponseEnum;
use crate::dto::request::UserRegistersWithPasswordRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_user_business::entity::user::User;

pub trait UserRegistersWithPasswordTrait: Send + Sync {
    fn handle(
        &self,
        _req: UserRegistersWithPasswordRequest,
        _user: User,
    ) -> Result<UserRegistersWithPasswordOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
