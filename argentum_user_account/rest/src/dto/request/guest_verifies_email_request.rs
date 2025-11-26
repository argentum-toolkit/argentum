use crate::dto::params::GuestVerifiesEmailParams;
use crate::dto::schema::EmailVerificationSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct GuestVerifiesEmailRequest {
    pub body: EmailVerificationSchema,
    pub params: GuestVerifiesEmailParams,
}

impl HttpRequest for GuestVerifiesEmailRequest {
    type Body = EmailVerificationSchema;
    type Params = GuestVerifiesEmailParams;

    fn new(body: EmailVerificationSchema, params: GuestVerifiesEmailParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
