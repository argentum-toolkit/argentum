use crate::api::dto::request::UserRegistersWithPasswordRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_rest_infrastructure::data_type::HttpResponse;

pub trait UserRegistersWithPasswordTrait: Send + Sync {
    fn handle(&self, _req: UserRegistersWithPasswordRequest) -> Result<HttpResponse, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
