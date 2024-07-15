use argentum_rest_infrastructure::data_type::HttpPathParams;
use serde::Deserialize;
use serde_valid::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserLoginsWithPasswordPathParams {}

impl UserLoginsWithPasswordPathParams {
    pub fn new() -> Self {
        Self {}
    }
}

impl HttpPathParams for UserLoginsWithPasswordPathParams {}
