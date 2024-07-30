use crate::dto::params::GetUserParams;
use argentum_rest_infrastructure::data_type::EmptyRequestBody;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct GetUserRequest {
    pub body: EmptyRequestBody,
    pub params: GetUserParams,
}

impl HttpRequest for GetUserRequest {
    type Body = EmptyRequestBody;
    type Params = GetUserParams;

    fn new(body: EmptyRequestBody, params: GetUserParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
