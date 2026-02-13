use argentum_rest_infrastructure::data_type::HttpPathParams;
use serde::Deserialize;
use serde_valid::Validate;

#[derive(Debug, Deserialize, Validate, Clone, PartialEq, Default)]
pub struct GuestVerifiesEmailPathParams {}

impl GuestVerifiesEmailPathParams {
    pub fn new() -> Self {
        Self {}
    }
}

impl HttpPathParams for GuestVerifiesEmailPathParams {}
