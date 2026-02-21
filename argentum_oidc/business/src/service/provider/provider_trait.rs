use async_trait::async_trait;

use crate::{data_type::ExternalUserInfo, service::provider::ProviderError};

#[async_trait]
pub trait ProviderTrait: Send + Sync {
    fn get_provider_name(&self) -> &'static str;
    async fn exchange_code(&self, code: &str) -> Result<String, ProviderError>;
    async fn get_user_info(&self, token: &str) -> Result<ExternalUserInfo, ProviderError>;
}
