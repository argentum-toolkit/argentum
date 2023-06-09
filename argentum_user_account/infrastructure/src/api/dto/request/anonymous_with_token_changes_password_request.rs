use crate::api::dto::schema::AnonymousRestoresPasswordWithTokenParams;
use argentum_rest_infrastructure::data_type::HttpRequest;
use argentum_user_account_api::models::ChangePasswordSchema;

pub struct AnonymousWithTokenChangesPasswordRequest {
    pub body: ChangePasswordSchema,
    pub params: AnonymousRestoresPasswordWithTokenParams,
}

impl HttpRequest for AnonymousWithTokenChangesPasswordRequest {
    type Body = ChangePasswordSchema;
    type Params = AnonymousRestoresPasswordWithTokenParams;

    fn new(body: ChangePasswordSchema, params: AnonymousRestoresPasswordWithTokenParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
