use crate::dto::params::UserLoginsWithPasswordParams;
use crate::dto::schema::LoginWithPasswordSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct UserLoginsWithPasswordRequest {
    pub body: LoginWithPasswordSchema,
    pub params: UserLoginsWithPasswordParams,
}

impl HttpRequest for UserLoginsWithPasswordRequest {
    type Body = LoginWithPasswordSchema;
    type Params = UserLoginsWithPasswordParams;

    fn new(body: LoginWithPasswordSchema, params: UserLoginsWithPasswordParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
