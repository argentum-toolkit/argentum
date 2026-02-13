use crate::dto::path_params::ExchangeCodePathParams;
use argentum_rest_infrastructure::data_type::EmptyHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams};

//TODO: query params generator
pub struct ExchangeCodeParams {
    pub headers: EmptyHeaderParams,
    pub path: ExchangeCodePathParams,
    pub query: EmptyQueryParams,
}

impl HttpParams for ExchangeCodeParams {
    type Headers = EmptyHeaderParams;
    type Path = ExchangeCodePathParams;
    type Query = EmptyQueryParams;

    fn new(path: Self::Path, query: Self::Query, headers: Self::Headers) -> Self {
        Self {
            path,
            query,
            headers,
        }
    }

    fn path(&self) -> &Self::Path {
        &self.path
    }

    fn query(&self) -> &Self::Query {
        &self.query
    }

    fn headers(&self) -> &Self::Headers {
        &self.headers
    }
}
