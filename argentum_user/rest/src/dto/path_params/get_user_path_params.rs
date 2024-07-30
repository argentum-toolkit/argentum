use argentum_rest_infrastructure::data_type::HttpPathParams;
use serde::Deserialize;
use serde_valid::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct GetUserPathParams {
    pub user_id: uuid::Uuid,
}

impl GetUserPathParams {
    pub fn new(user_id: uuid::Uuid) -> Self {
        Self { user_id }
    }
}

impl HttpPathParams for GetUserPathParams {}
