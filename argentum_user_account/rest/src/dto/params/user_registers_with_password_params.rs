use crate::dto::path_params::UserRegistersWithPasswordPathParams;
use argentum_rest_infrastructure::data_type::AuthHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams};

pub struct UserRegistersWithPasswordParams {
    pub headers: AuthHeaderParams,
    pub path: UserRegistersWithPasswordPathParams,
    pub query: EmptyQueryParams,
}

impl HttpParams for UserRegistersWithPasswordParams {
    type Headers = AuthHeaderParams;
    type Path = UserRegistersWithPasswordPathParams;
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
