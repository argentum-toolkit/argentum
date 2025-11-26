use argentum_standard_business::data_type::id::Id;

use std::sync::Arc;

use crate::{service::AuthCodeExchangerTrait, use_case::UserExchangesCodeToTokenUc};

pub struct OidcBusinessDiC<I>
where
    I: Id,
{
    // Public services
    pub user_exchanges_code_to_token_uc: Arc<UserExchangesCodeToTokenUc<I>>,
}

pub struct OidcBusinessDiCBuilder<I>
where
    I: Id,
{
    auth_code_exchanger: Option<Arc<dyn AuthCodeExchangerTrait<UserId = I>>>,
}

impl<I> OidcBusinessDiCBuilder<I>
where
    I: Id,
{
    pub fn new() -> Self {
        Self {
            auth_code_exchanger: None,
        }
    }

    pub fn with_auth_code_exchanger(
        &mut self,
        auth_code_exchanger: Arc<dyn AuthCodeExchangerTrait<UserId = I>>,
    ) -> &mut Self {
        self.auth_code_exchanger = Some(auth_code_exchanger);

        self
    }

    pub fn build(&self) -> Result<OidcBusinessDiC<I>, String> {
        let auth_code_exchanger = self
            .auth_code_exchanger
            .clone()
            .ok_or("auth_code_exchanger is not initialized")?;

        let user_exchanges_code_to_token_uc =
            Arc::new(UserExchangesCodeToTokenUc::new(auth_code_exchanger));

        Ok(OidcBusinessDiC {
            user_exchanges_code_to_token_uc,
        })
    }
}
