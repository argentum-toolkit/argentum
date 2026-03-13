use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::{entity::user::Guest, service::GuestBinder};

use crate::{
    data_type::ExchangedTokenData,
    service::code_exchanger::{CodeExchangerError, CodeExchangerTrait},
};

pub struct UserExchangesCodeToTokenUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    auth_code_exchanger: Arc<dyn CodeExchangerTrait<UserId = I>>,
    guest_binder: Arc<GuestBinder<I, L>>,
}

impl<I, L> UserExchangesCodeToTokenUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        auth_code_exchanger: Arc<dyn CodeExchangerTrait<UserId = I>>,
        guest_binder: Arc<GuestBinder<I, L>>,
    ) -> Self {
        Self {
            auth_code_exchanger,
            guest_binder,
        }
    }

    pub async fn execute(
        &self,
        guest: Guest<I>,
        code: &str,
        provider_name: &str,
    ) -> Result<ExchangedTokenData<I>, CodeExchangerError> {
        let data = self
            .auth_code_exchanger
            .exchange_code(code, provider_name)
            .await?;

        self.guest_binder
            .bind_and_logout_guest(Some(guest), data.user_id)
            .await;

        Ok(data)
    }
}
