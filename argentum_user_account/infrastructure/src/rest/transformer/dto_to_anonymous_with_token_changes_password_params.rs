use crate::api::dto::request::AnonymousWithTokenChangesPasswordRequest;
use argentum_rest_infrastructure::data_type::error::HttpError;

pub struct DtoToAnonymousWithTokenChangesPasswordParams {}

impl DtoToAnonymousWithTokenChangesPasswordParams {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(
        &self,
        req: AnonymousWithTokenChangesPasswordRequest,
    ) -> Result<(String, String), HttpError> {
        Ok((req.body.token, req.body.password))
    }
}
impl Default for DtoToAnonymousWithTokenChangesPasswordParams {
    fn default() -> Self {
        Self::new()
    }
}
