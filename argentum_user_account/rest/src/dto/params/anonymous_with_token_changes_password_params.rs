use argentum_rest_infrastructure::data_type::AuthHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyPathParams, HttpParams};

pub struct AnonymousWithTokenChangesPasswordParams {
    pub path: EmptyPathParams,
    pub headers: AuthHeaderParams,
}

impl HttpParams for AnonymousWithTokenChangesPasswordParams {
    type Path = EmptyPathParams;
    type Headers = AuthHeaderParams;

    fn new(path: Self::Path, headers: Self::Headers) -> Self {
        Self { path, headers }
    }

    fn path(&self) -> &Self::Path {
        &self.path
    }

    fn headers(&self) -> &Self::Headers {
        &self.headers
    }
}
