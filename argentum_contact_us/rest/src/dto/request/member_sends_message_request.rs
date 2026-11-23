use crate::dto::params::MemberSendsMessageParams;
use crate::dto::schema::MemberSendsMessageSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct MemberSendsMessageRequest {
    pub body: MemberSendsMessageSchema,
    pub params: MemberSendsMessageParams,
}

impl HttpRequest for MemberSendsMessageRequest {
    type Body = MemberSendsMessageSchema;
    type Params = MemberSendsMessageParams;

    fn new(body: MemberSendsMessageSchema, params: MemberSendsMessageParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
