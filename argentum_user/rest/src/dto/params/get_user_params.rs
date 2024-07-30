use crate::dto::path_params::GetUserPathParams;
use argentum_rest_infrastructure::data_type::AuthHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams};

pub struct GetUserParams {
    pub headers: AuthHeaderParams,
    pub path: GetUserPathParams,
    pub query: EmptyQueryParams,
}

impl HttpParams for GetUserParams {
    type Headers = AuthHeaderParams;
    type Path = GetUserPathParams;
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
