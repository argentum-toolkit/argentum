use crate::dto::params::ExchangeCodeParams;
use crate::dto::schema::ExchangeCodeSchema;
use argentum_rest_infrastructure::data_type::HttpRequest;

pub struct ExchangeCodeRequest {
    pub body: ExchangeCodeSchema,
    pub params: ExchangeCodeParams,
}

impl HttpRequest for ExchangeCodeRequest {
    type Body = ExchangeCodeSchema;
    type Params = ExchangeCodeParams;

    fn new(body: ExchangeCodeSchema, params: ExchangeCodeParams) -> Self {
        Self { body, params }
    }

    fn body(&self) -> &Self::Body {
        &self.body
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
