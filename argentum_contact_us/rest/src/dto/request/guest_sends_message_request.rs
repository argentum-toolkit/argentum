use crate::dto::params::GuestSendsMessageParams;
use crate::dto::schema::GuestSendsMessageSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct GuestSendsMessageRequest {
    pub body: GuestSendsMessageSchema,
    pub params: GuestSendsMessageParams,
}

impl HttpRequest for GuestSendsMessageRequest {
    type Body = GuestSendsMessageSchema;
    type Params = GuestSendsMessageParams;

    fn new(body: GuestSendsMessageSchema, params: GuestSendsMessageParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
