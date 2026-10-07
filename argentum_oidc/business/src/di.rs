use argentum_event_business::EventDispatcher;
use argentum_log_business::LoggerTrait;
use argentum_standard_business::{
    data_type::id::{Id, IdFactory},
    di_container::Container,
};
use argentum_user_account_business::token::MemberGeneratorTrait;
use argentum_user_business::{
    repository::{
        session_repository::SessionRepositoryTrait, user_repository::MemberRepositoryTrait,
    },
    service::GuestBinder,
};

use std::{collections::HashMap, sync::Arc};

use crate::{
    repository::CredentialRepositoryTrait,
    service::{
        code_exchanger::{CodeExchangerTrait, DefaultCodeExchanger},
        provider::ProviderStrategyTrait,
    },
    use_case::UserExchangesCodeToTokenUc,
};

pub fn oidc_business_provide<I, L>(container: &mut Container)
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    container.provide_factory::<dyn CodeExchangerTrait<UserId = I>, _>(|c| {
        let provider_strategies = c.resolve::<HashMap<String, Arc<dyn ProviderStrategyTrait>>>()?;
        let member_repository = c.resolve::<dyn MemberRepositoryTrait<Id = I>>()?;
        let credential_repository = c.resolve::<dyn CredentialRepositoryTrait<UserId = I>>()?;
        let session_repository = c.resolve::<dyn SessionRepositoryTrait<Id = I>>()?;
        let member_token_generator = c.resolve::<dyn MemberGeneratorTrait<Id = I>>()?;
        let id_factory = c.resolve::<dyn IdFactory<Id = I>>()?;
        let logger = c.resolve::<L>()?;
        let event_dispatcher = c.resolve::<EventDispatcher>()?;

        Ok(Arc::new(DefaultCodeExchanger::new(
            (*provider_strategies).clone(),
            member_repository,
            credential_repository,
            session_repository,
            member_token_generator,
            id_factory,
            logger,
            event_dispatcher,
        )))
    });

    container.provide_factory::<UserExchangesCodeToTokenUc<I, L>, _>(|c| {
        let code_exchanger = c.resolve::<dyn CodeExchangerTrait<UserId = I>>()?;
        let guest_binder = c.resolve::<GuestBinder<I, L>>()?;

        Ok(Arc::new(UserExchangesCodeToTokenUc::new(
            code_exchanger,
            guest_binder,
        )))
    });
}
