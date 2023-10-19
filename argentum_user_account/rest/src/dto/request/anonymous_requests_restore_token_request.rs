use argentum_rest_infrastructure::data_type::HttpRequest;
use crate::dto::params::AnonymousRequestsRestoreTokenParams;
use crate::dto::schema::RequestRestoreTokenSchema;

pub struct AnonymousRequestsRestoreTokenRequest {
    pub body: RequestRestoreTokenSchema,
    pub params: AnonymousRequestsRestoreTokenParams,
}

impl HttpRequest for AnonymousRequestsRestoreTokenRequest {
    type Body = RequestRestoreTokenSchema;
    type Params = AnonymousRequestsRestoreTokenParams;

    fn new(body: RequestRestoreTokenSchema, params: AnonymousRequestsRestoreTokenParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
