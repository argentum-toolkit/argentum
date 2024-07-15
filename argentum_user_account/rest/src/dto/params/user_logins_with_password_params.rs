use crate::dto::path_params::UserLoginsWithPasswordPathParams;
use argentum_rest_infrastructure::data_type::AuthHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams};

pub struct UserLoginsWithPasswordParams {
    pub headers: AuthHeaderParams,
    pub path: UserLoginsWithPasswordPathParams,
    pub query: EmptyQueryParams,
}

impl HttpParams for UserLoginsWithPasswordParams {
    type Headers = AuthHeaderParams;
    type Path = UserLoginsWithPasswordPathParams;
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
