use argentum_rest_infrastructure::data_type::AuthHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyPathParams, EmptyQueryParams, HttpParams};

pub struct AnonymousRequestsRestoreTokenParams {
    pub headers: AuthHeaderParams,
    pub path: EmptyPathParams,
    pub query: EmptyQueryParams,
}

impl HttpParams for AnonymousRequestsRestoreTokenParams {
    type Headers = AuthHeaderParams;
    type Path = EmptyPathParams;
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
