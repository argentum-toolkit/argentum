use std::sync::Arc;

use crate::{
    data_type::ExchangedTokenData,
    entity::Credential,
    repository::CredentialRepositoryTrait,
    service::{
        code_exchanger::{CodeExchangerError, CodeExchangerTrait},
        provider::ProviderTrait,
    },
};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_user_account_business::token::GeneratorTrait;
use argentum_user_business::{
    data_type::Name,
    entity::{session::Session, user::AuthenticatedUser},
    repository::{
        session_repository::SessionRepositoryTrait,
        user_repository::AuthenticatedUserRepositoryTrait,
    },
};
use async_trait::async_trait;
use chrono::Utc;

pub struct DefaultCodeExchanger<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    provider: Arc<dyn ProviderTrait>,
    member_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>,
    credential_repository: Arc<dyn CredentialRepositoryTrait<UserId = I>>,
    session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
    token_generator: Arc<dyn GeneratorTrait<Id = I>>,
    id_factory: Arc<dyn IdFactory<Id = I>>,
    logger: Arc<L>,
}

impl<I, L> DefaultCodeExchanger<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        provider: Arc<dyn ProviderTrait>,
        member_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>,
        credential_repository: Arc<dyn CredentialRepositoryTrait<UserId = I>>,
        session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
        token_generator: Arc<dyn GeneratorTrait<Id = I>>,
        id_factory: Arc<dyn IdFactory<Id = I>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            provider,
            member_repository,
            credential_repository,
            session_repository,
            token_generator,
            id_factory,
            logger,
        }
    }
}

#[async_trait]
impl<I, L> CodeExchangerTrait for DefaultCodeExchanger<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    type UserId = I;

    async fn exchange_code(
        &self,
        code: &str,
    ) -> Result<ExchangedTokenData<Self::UserId>, CodeExchangerError> {
        let external_access_token = self.provider.exchange_code(code).await?;

        let gh_user = self.provider.get_user_info(&external_access_token).await?;

        let gh_user_id = gh_user.id;

        let credential_result = self
            .credential_repository
            .find_by_external_id(&gh_user_id, self.provider.get_provider_name())
            .await;

        let cred = match credential_result {
            Ok(c) => c,
            Err(e) => {
                self.logger
                    .error(format!("Can't get member from db. Error: {}", e));
                return Err(CodeExchangerError::Other(Some(Arc::new(e))));
            }
        };

        let credential = match cred {
            Some(c) => c,
            None => {
                let email = gh_user.email;

                let user: Option<AuthenticatedUser<I>> = None;
                let user = match user {
                    Some(u) => u,
                    None => {
                        let user_id = self.id_factory.create();
                        let name = Name::new(gh_user.first_name, gh_user.last_name, None);

                        let user = AuthenticatedUser::new(user_id, name, email);
                        if let Err(e) = self.member_repository.create(&user).await {
                            return Err(CodeExchangerError::Other(Some(Arc::new(e))));
                        }

                        user
                    }
                };

                let external_id = user.id.to_string();
                let created_at = Utc::now();

                let credential = Credential::new(
                    &external_id,
                    user.id,
                    self.provider.get_provider_name(),
                    created_at,
                );

                if let Err(e) = self.credential_repository.insert(&credential).await {
                    return Err(CodeExchangerError::Other(Some(Arc::new(e))));
                }

                credential
            }
        };

        let access_token = self.token_generator.generate(credential.user_id);
        let session = Session::new(self.id_factory.create(), credential.user_id, &access_token);

        if let Err(e) = self.session_repository.save(&session).await {
            return Err(CodeExchangerError::Other(Some(Arc::new(e))));
        }

        return Ok(ExchangedTokenData {
            access_token,
            user_id: credential.user_id,
        });
    }
}
