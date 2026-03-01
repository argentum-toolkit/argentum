use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_user_account_rest::dto::request::GuestWithTokenChangesPasswordRequest;

pub struct DtoToGuestWithTokenChangesPasswordParams {}

impl DtoToGuestWithTokenChangesPasswordParams {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(
        &self,
        req: GuestWithTokenChangesPasswordRequest,
    ) -> Result<(String, String), HttpError> {
        Ok((req.body.token, req.body.password))
    }
}
impl Default for DtoToGuestWithTokenChangesPasswordParams {
    fn default() -> Self {
        Self::new()
    }
}
