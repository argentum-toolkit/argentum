use crate::dto::path_params::GuestVerifyEmailPathParams;
use argentum_rest_infrastructure::data_type::AuthHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams};

//TODO: query params generator
pub struct GuestVerifyEmailParams {
    pub headers: AuthHeaderParams,
    pub path: GuestVerifyEmailPathParams,
    pub query: EmptyQueryParams,
}

impl HttpParams for GuestVerifyEmailParams {
    type Headers = AuthHeaderParams;
    type Path = GuestVerifyEmailPathParams;
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
