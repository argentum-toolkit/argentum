use crate::dto::params::MemberLoginsWithPasswordParams;
use crate::dto::schema::LoginWithPasswordSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct MemberLoginsWithPasswordRequest {
    pub body: LoginWithPasswordSchema,
    pub params: MemberLoginsWithPasswordParams,
}

impl HttpRequest for MemberLoginsWithPasswordRequest {
    type Body = LoginWithPasswordSchema;
    type Params = MemberLoginsWithPasswordParams;

    fn new(body: LoginWithPasswordSchema, params: MemberLoginsWithPasswordParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
