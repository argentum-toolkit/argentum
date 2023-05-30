use argentum_rest_infrastructure::data_type::{EmptyPathParams, HttpParams};

pub struct UserRegistersWithPasswordParams {
    pub path: EmptyPathParams,
}

impl HttpParams for UserRegistersWithPasswordParams {
    type Path = EmptyPathParams;

    fn new(path: Self::Path) -> Self {
        Self { path }
    }

    fn path(&self) -> &Self::Path {
        &self.path
    }
}
