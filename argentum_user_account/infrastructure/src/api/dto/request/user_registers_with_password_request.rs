use crate::api::dto::schema::UserRegistersWithPasswordParams;
use argentum_rest_infrastructure::data_type::HttpRequest;
use argentum_user_account_api::models::RegistrationWithPasswordSchema;

pub struct UserRegistersWithPasswordRequest {
    pub body: RegistrationWithPasswordSchema,
    pub params: UserRegistersWithPasswordParams,
}

impl HttpRequest for UserRegistersWithPasswordRequest {
    type Body = RegistrationWithPasswordSchema;
    type Params = UserRegistersWithPasswordParams;

    fn new(body: RegistrationWithPasswordSchema, params: UserRegistersWithPasswordParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
