use crate::dto::request::UserLoginsWithPasswordRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_rest_infrastructure::data_type::HttpResponse;
use argentum_user_business::entity::user::User;

pub trait UserLoginsWithPasswordTrait: Send + Sync {
    fn handle(
        &self,
        _req: UserLoginsWithPasswordRequest,
        _user: User,
    ) -> Result<HttpResponse, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
