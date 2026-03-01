use crate::dto::params::GuestRegistersParams;
use argentum_rest_infrastructure::data_type::EmptyRequestBody;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct GuestRegistersRequest {
    pub body: EmptyRequestBody,
    pub params: GuestRegistersParams,
}

impl HttpRequest for GuestRegistersRequest {
    type Body = EmptyRequestBody;
    type Params = GuestRegistersParams;

    fn new(body: EmptyRequestBody, params: GuestRegistersParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
