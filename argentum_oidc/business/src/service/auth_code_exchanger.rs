use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;

use crate::data_type::ExchangedTokenData;

#[async_trait]
pub trait AuthCodeExchangerTrait: Send + Sync {
    type UserId: Id;

    async fn exchange_code(
        &self,
        code: &str,
    ) -> Result<ExchangedTokenData<Self::UserId>, AuthCodeExchangerError>;
}

#[derive(thiserror::Error, Debug)]
pub enum AuthCodeExchangerError {
    #[error("Cant provide token data. Error: `{0}`")]
    TokenData(String),

    #[error("Can't exchange code. Network error")]
    Network,

    #[error("Can't exchange code. Bad response status: `{0}`")]
    OidcBadStatus(u16),
}
