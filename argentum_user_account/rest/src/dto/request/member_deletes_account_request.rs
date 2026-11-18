use crate::dto::params::MemberDeletesAccountParams;
use crate::dto::schema::DeleteAccountSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct MemberDeletesAccountRequest {
    pub body: DeleteAccountSchema,
    pub params: MemberDeletesAccountParams,
}

impl HttpRequest for MemberDeletesAccountRequest {
    type Body = DeleteAccountSchema;
    type Params = MemberDeletesAccountParams;

    fn new(body: DeleteAccountSchema, params: MemberDeletesAccountParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
