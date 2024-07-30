use crate::dto::params::AnonymousWithTokenChangesPasswordParams;
use crate::dto::schema::ChangePasswordSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct AnonymousWithTokenChangesPasswordRequest {
    pub body: ChangePasswordSchema,
    pub params: AnonymousWithTokenChangesPasswordParams,
}

impl HttpRequest for AnonymousWithTokenChangesPasswordRequest {
    type Body = ChangePasswordSchema;
    type Params = AnonymousWithTokenChangesPasswordParams;

    fn new(body: ChangePasswordSchema, params: AnonymousWithTokenChangesPasswordParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
