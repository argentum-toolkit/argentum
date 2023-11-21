use argentum_rest_infrastructure::data_type::AuthHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyPathParams, EmptyQueryParams, HttpParams};

pub struct AnonymousRequestsRestoreTokenParams {
    pub path: EmptyPathParams,
    pub query: EmptyQueryParams,
    pub headers: AuthHeaderParams,
}

impl HttpParams for AnonymousRequestsRestoreTokenParams {
    type Path = EmptyPathParams;

    type Query = EmptyQueryParams;

    type Headers = AuthHeaderParams;

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
