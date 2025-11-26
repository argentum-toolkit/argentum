use std::sync::Arc;

use argentum_standard_business::data_type::id::Id;

use crate::{
    data_type::ExchangedTokenData, service::AuthCodeExchangerTrait,
    use_case::error::ExchangesCodeToTokenError,
};

pub struct UserExchangesCodeToTokenUc<I>
where
    I: Id,
{
    auth_code_exchanger: Arc<dyn AuthCodeExchangerTrait<UserId = I>>,
}

impl<I> UserExchangesCodeToTokenUc<I>
where
    I: Id,
{
    pub fn new(auth_code_exchanger: Arc<dyn AuthCodeExchangerTrait<UserId = I>>) -> Self {
        Self {
            auth_code_exchanger,
        }
    }

    pub async fn execute(
        &self,
        code: &str,
    ) -> Result<ExchangedTokenData<I>, ExchangesCodeToTokenError> {
        Ok(self.auth_code_exchanger.exchange_code(code).await?)
    }
}
