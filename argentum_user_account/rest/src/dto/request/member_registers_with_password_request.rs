use crate::dto::params::MemberRegistersWithPasswordParams;
use crate::dto::schema::RegistrationWithPasswordSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct MemberRegistersWithPasswordRequest {
    pub body: RegistrationWithPasswordSchema,
    pub params: MemberRegistersWithPasswordParams,
}

impl HttpRequest for MemberRegistersWithPasswordRequest {
    type Body = RegistrationWithPasswordSchema;
    type Params = MemberRegistersWithPasswordParams;

    fn new(
        body: RegistrationWithPasswordSchema,
        params: MemberRegistersWithPasswordParams,
    ) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
