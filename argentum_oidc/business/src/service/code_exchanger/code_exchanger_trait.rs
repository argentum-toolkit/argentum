use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;

use crate::{data_type::ExchangedTokenData, service::code_exchanger::CodeExchangerError};

#[async_trait]
pub trait CodeExchangerTrait: Send + Sync {
    type UserId: Id;

    async fn exchange_code(
        &self,
        code: &str,
        provider_name: &str,
    ) -> Result<ExchangedTokenData<Self::UserId>, CodeExchangerError>;
}
