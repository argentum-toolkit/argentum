use crate::dto::operation_response_enum::AnonymousWithTokenChangesPasswordOperationResponseEnum;
use crate::dto::request::AnonymousWithTokenChangesPasswordRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_user_business::entity::user::User;

pub trait AnonymousWithTokenChangesPasswordTrait: Send + Sync {
    fn handle(
        &self,
        _req: AnonymousWithTokenChangesPasswordRequest,
        _user: User,
    ) -> Result<AnonymousWithTokenChangesPasswordOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
