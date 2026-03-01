use crate::dto::path_params::MemberRegistersWithPasswordPathParams;
use argentum_rest_infrastructure::data_type::AuthHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams};

//TODO: query params generator
pub struct MemberRegistersWithPasswordParams {
    pub headers: AuthHeaderParams,
    pub path: MemberRegistersWithPasswordPathParams,
    pub query: EmptyQueryParams,
}

impl HttpParams for MemberRegistersWithPasswordParams {
    type Headers = AuthHeaderParams;
    type Path = MemberRegistersWithPasswordPathParams;
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
