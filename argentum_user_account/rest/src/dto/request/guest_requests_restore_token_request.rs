use crate::dto::params::GuestRequestsRestoreTokenParams;
use crate::dto::schema::RequestRestoreTokenSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct GuestRequestsRestoreTokenRequest {
    pub body: RequestRestoreTokenSchema,
    pub params: GuestRequestsRestoreTokenParams,
}

impl HttpRequest for GuestRequestsRestoreTokenRequest {
    type Body = RequestRestoreTokenSchema;
    type Params = GuestRequestsRestoreTokenParams;

    fn new(body: RequestRestoreTokenSchema, params: GuestRequestsRestoreTokenParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
