use crate::dto::params::GuestWithTokenChangesPasswordParams;
use crate::dto::schema::ChangePasswordSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct GuestWithTokenChangesPasswordRequest {
    pub body: ChangePasswordSchema,
    pub params: GuestWithTokenChangesPasswordParams,
}

impl HttpRequest for GuestWithTokenChangesPasswordRequest {
    type Body = ChangePasswordSchema;
    type Params = GuestWithTokenChangesPasswordParams;

    fn new(body: ChangePasswordSchema, params: GuestWithTokenChangesPasswordParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
