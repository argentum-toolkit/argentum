use crate::dto::path_params::GuestSendsMessagePathParams;
use argentum_rest_infrastructure::data_type::EmptyHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams};

//TODO: query params generator
pub struct GuestSendsMessageParams {
    pub headers: EmptyHeaderParams,
    pub path: GuestSendsMessagePathParams,
    pub query: EmptyQueryParams,
}

impl HttpParams for GuestSendsMessageParams {
    type Headers = EmptyHeaderParams;
    type Path = GuestSendsMessagePathParams;
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
