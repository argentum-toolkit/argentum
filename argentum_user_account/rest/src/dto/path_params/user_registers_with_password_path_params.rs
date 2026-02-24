
use serde::Deserialize;
use serde_valid::Validate;
use argentum_rest_infrastructure::data_type::HttpPathParams;

#[derive(Debug, Deserialize, Validate, Clone, PartialEq, Default)]
pub struct UserRegistersWithPasswordPathParams {
}

impl UserRegistersWithPasswordPathParams {
    pub fn new(
    ) -> Self {
        Self {
        }
    }
}


impl HttpPathParams for UserRegistersWithPasswordPathParams {}
