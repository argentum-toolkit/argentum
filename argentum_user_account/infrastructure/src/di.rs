use crate::db::repository::{PasswordCredentialRepository, RestorePasswordTokenRepository};
use crate::rest::handler::{
    AnonymousRegistersHandler, AnonymousRequestsRestoreTokenHandler,
    AnonymousWithTokenChangesPasswordHandler, MemberVerifiesEmailHandler,
    UserLoginsWithPasswordHandler, UserRegistersWithPasswordHandler,
};
use crate::rest::transformer::{
    DtoToAnonymousRequestsRestoreTokenParams, DtoToAnonymousWithTokenChangesPasswordParams,
    DtoToUserLoginsWithPasswordParams, DtoToUserRegistersWithPasswordParams,
};
use crate::token::{JwtTokenGenerator, StringTokenGenerator};
use argentum_encryption_business::password::{Encryptor, Validator};
use argentum_log_business::LoggerTrait;
use argentum_notification_business::NotifierTrait;
use argentum_rest_infrastructure::di::RestDiBuilder;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::di::{BusinessDi, UserAccountBusinessDiBuilder};
use argentum_user_account_business::repository::password_credential_repository::PasswordCredentialRepositoryTrait;
use argentum_user_account_business::service::PasswordToTokenExchangerTrait;
use argentum_user_account_business::use_case::member_confirms_email::MemberConfirmsEmailUcTrait;
use argentum_user_account_rest::ApiDiC;
use argentum_user_business::di::UserBusinessDiBuilder;

use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, Mutex};

pub struct UserAccountInfrastructureDi<I, L>
where
    I: Id + 'static,
    L: LoggerTrait,
{
    // Public services
    pub business_di: BusinessDi<I, L>,
    pub api_di: ApiDiC<I>,
}

pub struct UserAccountInfrastructureDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    connection_url: String,
    max_db_connections: u32,
    member_jwt_token_secret: String,
    member_jwt_token_ttl: i64,

    pub business_builder: Arc<Mutex<UserAccountBusinessDiBuilder<UniqueId, L>>>,
    rest_builder: Arc<RestDiBuilder<UniqueId, L>>,

    logger: Arc<L>,
    encryptor: Arc<dyn Encryptor>,
}

impl<L> UserAccountInfrastructureDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    pub fn new(
        product_name: &str,
        restore_password_token_ttl: u32,
        restore_password_front_url: &str,
        connection_url: &str,
        max_db_connections: u32,
        member_jwt_token_secret: &str,
        member_jwt_token_ttl: i64,

        user_business_builder: Arc<Mutex<UserBusinessDiBuilder<UniqueId, L>>>,

        rest_builder: Arc<RestDiBuilder<UniqueId, L>>,
        id_factory: Arc<dyn IdFactory<Id = UniqueId>>,

        logger: Arc<L>,
        notifier: Arc<dyn NotifierTrait<Id = UniqueId>>,
        validator: Arc<dyn Validator>,
        encryptor: Arc<dyn Encryptor>,
    ) -> Self {
        Self {
            connection_url: connection_url.into(),
            max_db_connections,
            member_jwt_token_secret: member_jwt_token_secret.into(),
            member_jwt_token_ttl,

            rest_builder,
            business_builder: Arc::new(Mutex::new(UserAccountBusinessDiBuilder::new(
                user_business_builder,
                product_name,
                restore_password_token_ttl,
                restore_password_front_url,
                id_factory.clone(),
                logger.clone(),
                notifier,
                validator,
            ))),
            logger,
            encryptor,
        }
    }

    pub async fn wit_default_services(&mut self) -> Result<&mut Self, String> {
        let pool = PgPoolOptions::new()
            .max_connections(self.max_db_connections)
            .min_connections(2)
            .connect(&self.connection_url)
            .await
            .map_err(|e| format!("Can't create PG connection pool. Error: {e}"))?;

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool, self.logger.clone()));

        let restore_password_token_repository =
            Arc::new(RestorePasswordTokenRepository::new(pg_adapter.clone()));

        let member_token_generator = Arc::new(JwtTokenGenerator::new(
            &self.member_jwt_token_secret,
            self.member_jwt_token_ttl,
        ));
        let guest_token_generator = Arc::new(StringTokenGenerator::new());

        let password_credential_repository = Arc::new(PasswordCredentialRepository::new(
            pg_adapter,
            self.encryptor.clone(),
        ));

        self.business_builder.lock().unwrap().with_default_services(
            password_credential_repository,
            restore_password_token_repository,
            guest_token_generator,
            member_token_generator,
        );

        Ok(self)
    }

    pub fn with_password_credential_repository(
        &mut self,
        password_credential_repository: Arc<dyn PasswordCredentialRepositoryTrait<Id = UniqueId>>,
    ) -> Result<&mut Self, String> {
        self.business_builder
            .lock()
            .unwrap()
            .with_password_credential_repository(password_credential_repository);

        Ok(self)
    }

    pub fn with_password_to_token_exchanger(
        &mut self,
        password_to_token_exchanger: Arc<dyn PasswordToTokenExchangerTrait<Id = UniqueId>>,
    ) -> Result<&mut Self, String> {
        self.business_builder
            .lock()
            .unwrap()
            .with_password_to_token_exchanger(password_to_token_exchanger);

        Ok(self)
    }

    pub fn with_member_confirms_email_uc(
        &mut self,
        member_confirms_email_uc: Arc<dyn MemberConfirmsEmailUcTrait<Id = UniqueId>>,
    ) -> Result<&mut Self, String> {
        self.business_builder
            .lock()
            .unwrap()
            .with_member_confirms_email_uc(member_confirms_email_uc);

        Ok(self)
    }

    pub fn build(&self) -> Result<UserAccountInfrastructureDi<UniqueId, L>, String> {
        let business_di = self.business_builder.lock().unwrap().build()?;

        let anonymous_registers_handler = Arc::new(AnonymousRegistersHandler::new(
            business_di.anonymous_registers_uc.clone(),
        ));

        let dto_to_user_registers_with_password_params =
            Arc::new(DtoToUserRegistersWithPasswordParams::new());

        let user_registers_with_password_handler = Arc::new(UserRegistersWithPasswordHandler::new(
            business_di.user_registers_with_password_uc.clone(),
            dto_to_user_registers_with_password_params,
        ));

        let dto_to_user_logins_with_password_params =
            Arc::new(DtoToUserLoginsWithPasswordParams::new());

        let user_logins_with_password_handler = Arc::new(UserLoginsWithPasswordHandler::new(
            business_di.user_logins_with_password_uc.clone(),
            dto_to_user_logins_with_password_params,
        ));

        let dto_to_anonymous_with_token_changes_password_params =
            Arc::new(DtoToAnonymousWithTokenChangesPasswordParams::new());

        let anonymous_with_token_changes_password_handler =
            Arc::new(AnonymousWithTokenChangesPasswordHandler::new(
                business_di.anonymous_with_token_changes_password_uc.clone(),
                dto_to_anonymous_with_token_changes_password_params,
            ));

        let dto_to_anonymous_requests_restore_token_params =
            Arc::new(DtoToAnonymousRequestsRestoreTokenParams::new());

        let anonymous_requests_restore_token_handler =
            Arc::new(AnonymousRequestsRestoreTokenHandler::new(
                business_di.anonymous_requests_restore_token_uc.clone(),
                self.logger.clone(),
                dto_to_anonymous_requests_restore_token_params,
            ));

        let member_verifies_email_handler = Arc::new(MemberVerifiesEmailHandler::new(
            business_di.member_confirms_email_uc.clone(),
            self.logger.clone(),
        ));

        let rest_di = self.rest_builder.build();

        let api_di = ApiDiC::new(
            "/api/v1".to_string(),
            rest_di.request_transformer.clone(),
            rest_di.bearer_authenticator.clone(),
            anonymous_registers_handler,
            anonymous_requests_restore_token_handler,
            anonymous_with_token_changes_password_handler,
            member_verifies_email_handler,
            user_logins_with_password_handler,
            user_registers_with_password_handler,
            rest_di.error_pre_handler.clone(),
        );

        Ok(UserAccountInfrastructureDi {
            business_di,
            api_di,
        })
    }
}
