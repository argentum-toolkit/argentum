
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_rest_infrastructure::data_type::HttpResponse;


pub trait AnonymousRegistersTrait: Send + Sync {
    fn handle(
        &self,
        
        
    ) -> Result<HttpResponse, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
