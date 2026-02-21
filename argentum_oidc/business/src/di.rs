use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::service::GuestBinder;

use std::sync::Arc;

use crate::{
    repository::CredentialRepositoryTrait, service::code_exchanger::CodeExchangerTrait,
    use_case::UserExchangesCodeToTokenUc,
};

pub struct OidcBusinessDiC<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    // Public services
    pub user_exchanges_code_to_token_uc: Arc<UserExchangesCodeToTokenUc<I, L>>,
    pub credential_repository: Arc<dyn CredentialRepositoryTrait<UserId = I>>,
}

pub struct OidcBusinessDiCBuilder<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    auth_code_exchanger: Option<Arc<dyn CodeExchangerTrait<UserId = I>>>,
    credential_repository: Option<Arc<dyn CredentialRepositoryTrait<UserId = I>>>,
    guest_binder: Option<Arc<GuestBinder<I, L>>>,
}

impl<I, L> OidcBusinessDiCBuilder<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new() -> Self {
        Self {
            auth_code_exchanger: None,
            credential_repository: None,
            guest_binder: None,
        }
    }

    pub fn with_auth_code_exchanger(
        &mut self,
        auth_code_exchanger: Arc<dyn CodeExchangerTrait<UserId = I>>,
    ) -> &mut Self {
        self.auth_code_exchanger = Some(auth_code_exchanger);

        self
    }

    pub fn with_credential_repository(
        &mut self,
        credential_repository: Arc<dyn CredentialRepositoryTrait<UserId = I>>,
    ) -> &mut Self {
        self.credential_repository = Some(credential_repository);

        self
    }

    pub fn with_guest_binder(&mut self, guest_binder: Arc<GuestBinder<I, L>>) -> &mut Self {
        self.guest_binder = Some(guest_binder);

        self
    }

    pub fn build(&self) -> Result<OidcBusinessDiC<I, L>, String> {
        let auth_code_exchanger = self
            .auth_code_exchanger
            .clone()
            .ok_or("auth_code_exchanger is not initialized")?;

        let guest_binder = self
            .guest_binder
            .clone()
            .ok_or("guest_binder is not initialized")?;

        let user_exchanges_code_to_token_uc = Arc::new(UserExchangesCodeToTokenUc::new(
            auth_code_exchanger,
            guest_binder,
        ));

        let credential_repository = self
            .credential_repository
            .clone()
            .ok_or("credential_repository is not initialized")?;

        Ok(OidcBusinessDiC {
            user_exchanges_code_to_token_uc,
            credential_repository,
        })
    }
}

impl<I, L> Default for OidcBusinessDiCBuilder<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    fn default() -> Self {
        Self::new()
    }
}
