use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_user_account_business::di::UserAccountBusinessDiBuilder;
use argentum_user_business::di::UserBusinessDiBuilder;

use std::sync::{Arc, Mutex};

use crate::{
    repository::CredentialRepositoryTrait,
    service::{
        code_exchanger::{CodeExchangerTrait, DefaultCodeExchanger},
        provider::ProviderTrait,
    },
    use_case::UserExchangesCodeToTokenUc,
};

pub struct OidcBusinessDi<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    // Public services
    pub user_exchanges_code_to_token_uc: Arc<UserExchangesCodeToTokenUc<I, L>>,
}

pub struct OidcBusinessDiBuilder<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    user_business_builder: Arc<Mutex<UserBusinessDiBuilder<I, L>>>,
    user_account_business_builder: Arc<Mutex<UserAccountBusinessDiBuilder<I, L>>>,

    provider: Option<Arc<dyn ProviderTrait>>,
    code_exchanger: Option<Arc<dyn CodeExchangerTrait<UserId = I>>>,
    credential_repository: Option<Arc<dyn CredentialRepositoryTrait<UserId = I>>>,
    id_factory: Arc<dyn IdFactory<Id = I>>,
    logger: Arc<L>,
}

impl<I, L> OidcBusinessDiBuilder<I, L>
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    pub fn new(
        user_business_builder: Arc<Mutex<UserBusinessDiBuilder<I, L>>>,
        user_account_business_builder: Arc<Mutex<UserAccountBusinessDiBuilder<I, L>>>,

        id_factory: Arc<dyn IdFactory<Id = I>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            user_business_builder,
            user_account_business_builder,
            id_factory,
            logger,
            provider: None,
            code_exchanger: None,
            credential_repository: None,
        }
    }

    pub fn with_code_exchanger(
        &mut self,
        code_exchanger: Arc<dyn CodeExchangerTrait<UserId = I>>,
    ) -> &mut Self {
        self.code_exchanger = Some(code_exchanger);

        self
    }

    pub fn with_credential_repository(
        &mut self,
        credential_repository: Arc<dyn CredentialRepositoryTrait<UserId = I>>,
    ) -> &mut Self {
        self.credential_repository = Some(credential_repository);

        self
    }

    pub fn with_provider(&mut self, provider: Arc<dyn ProviderTrait>) -> &mut Self {
        self.provider = Some(provider);

        self
    }

    pub fn build(&self) -> Result<OidcBusinessDi<I, L>, String> {
        let user_business_di = self.user_business_builder.lock().unwrap().build()?;
        let user_account_business_di =
            self.user_account_business_builder.lock().unwrap().build()?;

        let code_exchanger = match self.code_exchanger.clone() {
            Some(c) => c,
            None => {
                let credential_repository = self
                    .credential_repository
                    .clone()
                    .ok_or("credential_repository is not initialized!")?;

                let provider = self.provider.clone().ok_or("provider is not initialized")?;

                Arc::new(DefaultCodeExchanger::new(
                    provider,
                    user_business_di.authenticated_user_repository.clone(),
                    credential_repository.clone(),
                    user_business_di.session_repository.clone(),
                    user_account_business_di.member_token_generator.clone(),
                    self.id_factory.clone(),
                    self.logger.clone(),
                ))
            }
        };

        let user_exchanges_code_to_token_uc = Arc::new(UserExchangesCodeToTokenUc::new(
            code_exchanger,
            user_business_di.guest_binder.clone(),
        ));

        Ok(OidcBusinessDi {
            user_exchanges_code_to_token_uc,
        })
    }
}
