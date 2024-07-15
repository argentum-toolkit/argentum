use crate::dto::params::AnonymousRegistersParams;
use argentum_rest_infrastructure::data_type::EmptyRequestBody;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct AnonymousRegistersRequest {
    pub body: EmptyRequestBody,
    pub params: AnonymousRegistersParams,
}

impl HttpRequest for AnonymousRegistersRequest {
    type Body = EmptyRequestBody;
    type Params = AnonymousRegistersParams;

    fn new(body: EmptyRequestBody, params: AnonymousRegistersParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
