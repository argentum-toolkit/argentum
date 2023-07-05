    use argentum_rest_infrastructure::data_type::EmptyHeaderParams;
use argentum_rest_infrastructure::data_type::{EmptyPathParams, HttpParams};

pub struct AnonymousRegistersParams {
    pub path: EmptyPathParams,
pub headers: EmptyHeaderParams,
}

impl HttpParams for AnonymousRegistersParams {
    type Path = EmptyPathParams;
    type Headers = EmptyHeaderParams;

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
