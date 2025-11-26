use crate::dto::params::MemberVerifiesEmailParams;
use crate::dto::schema::EmailVerificationSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct MemberVerifiesEmailRequest {
    pub body: EmailVerificationSchema,
    pub params: MemberVerifiesEmailParams,
}

impl HttpRequest for MemberVerifiesEmailRequest {
    type Body = EmailVerificationSchema;
    type Params = MemberVerifiesEmailParams;

    fn new(body: EmailVerificationSchema, params: MemberVerifiesEmailParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
