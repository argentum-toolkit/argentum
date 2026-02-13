use crate::dto::params::GuestVerifyEmailParams;
use crate::dto::schema::EmailVerificationSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct GuestVerifyEmailRequest {
    pub body: EmailVerificationSchema,
    pub params: GuestVerifyEmailParams,
}

impl HttpRequest for GuestVerifyEmailRequest {
    type Body = EmailVerificationSchema;
    type Params = GuestVerifyEmailParams;

    fn new(body: EmailVerificationSchema, params: GuestVerifyEmailParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
