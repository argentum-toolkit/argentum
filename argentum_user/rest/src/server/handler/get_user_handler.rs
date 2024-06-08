use crate::dto::operation_response_enum::GetUserOperationResponseEnum;
use crate::dto::request::GetUserRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_user_business::entity::user::User;

pub trait GetUserTrait: Send + Sync {
    fn handle(
        &self,
        _req: GetUserRequest,
        _user: User,
    ) -> Result<GetUserOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
