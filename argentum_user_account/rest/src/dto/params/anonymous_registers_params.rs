use crate::dto::path_params::AnonymousRegistersPathParams;
use argentum_rest_infrastructure::data_type::EmptyHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams};

pub struct AnonymousRegistersParams {
    pub headers: EmptyHeaderParams,
    pub path: AnonymousRegistersPathParams,
    pub query: EmptyQueryParams,
}

impl HttpParams for AnonymousRegistersParams {
    type Headers = EmptyHeaderParams;
    type Path = AnonymousRegistersPathParams;
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
