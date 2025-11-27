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
use crate::token::StringTokenGenerator;
use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_log_business::LoggerTrait;
use argentum_notification_business::NotificatorTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::di::UserAccountBusinessDiCBuilder;
use argentum_user_account_business::repository::password_credential_checker::PasswordCredentialChecker;
use argentum_user_account_business::repository::password_credential_repository::PasswordCredentialRepositoryTrait;
use argentum_user_account_business::service::{
    DefaultPasswordToTokenExchanger, PasswordToTokenExchangerTrait,
};
use argentum_user_account_business::use_case::member_confirms_email::{
    DefaultMemberConfirmsEmailUc, MemberConfirmsEmailUcTrait,
};
use argentum_user_account_rest::server::handler::{
    AnonymousRegistersTrait, AnonymousWithTokenChangesPasswordTrait, MemberVerifiesEmailTrait,
    UserLoginsWithPasswordTrait, UserRegistersWithPasswordTrait,
};
use argentum_user_infrastructure::di::UserInfrastructureDiC;

use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct UserAccountInfrastructureDiC<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    // Public services
    pub anonymous_registers_handler: Arc<dyn AnonymousRegistersTrait<Id = I>>,
    pub anonymous_requests_restore_token_handler: Arc<AnonymousRequestsRestoreTokenHandler<I, L>>,
    pub anonymous_with_token_changes_password_handler:
        Arc<dyn AnonymousWithTokenChangesPasswordTrait<Id = I>>,
    pub user_registers_with_password_handler: Arc<dyn UserRegistersWithPasswordTrait<Id = I>>,
    pub user_logins_with_password_handler: Arc<dyn UserLoginsWithPasswordTrait<Id = I>>,
    pub member_verifies_email_handler: Arc<dyn MemberVerifiesEmailTrait<Id = I>>,
}

pub struct UserAccountInfrastructureDiCBuilder<L>
where
    L: LoggerTrait,
{
    user_infrastructure_di: Arc<UserInfrastructureDiC<UniqueId>>,
    business_builder: UserAccountBusinessDiCBuilder<UniqueId, L>,
    id_factory: Arc<dyn IdFactory<Id = UniqueId>>,
    logger: Arc<L>,
}

impl<L> UserAccountInfrastructureDiCBuilder<L>
where
    L: LoggerTrait + 'static,
{
    pub fn new(
        user_infrastructure_di: Arc<UserInfrastructureDiC<UniqueId>>,
        id_factory: Arc<dyn IdFactory<Id = UniqueId>>,

        // validator: Arc<dyn Validator>,
        logger: Arc<L>,
        notificator: Arc<dyn NotificatorTrait<Id = UniqueId>>,
    ) -> Self {
        Self {
            user_infrastructure_di,
            business_builder: UserAccountBusinessDiCBuilder::new(
                id_factory.clone(),
                logger.clone(),
                notificator,
            ),
            id_factory,
            logger,
        }
    }

    pub fn config<S>(
        &mut self,
        product_name: S,
        restore_password_token_ttl: u32,
        restore_password_front_url: S,
    ) -> &mut Self
    where
        S: Into<String>,
    {
        self.business_builder.config(
            product_name,
            restore_password_token_ttl,
            restore_password_front_url,
        );

        self
    }

    pub async fn services(
        &mut self,
        connection_url: &str,
        max_db_connections: u32,
        password_credential_repository: Option<
            Arc<dyn PasswordCredentialRepositoryTrait<Id = UniqueId>>,
        >,
        password_to_token_exchanger: Option<Arc<dyn PasswordToTokenExchangerTrait<Id = UniqueId>>>,
        member_confirms_email_uc: Option<Arc<dyn MemberConfirmsEmailUcTrait<Id = UniqueId>>>,

        logger: Arc<L>,
    ) -> Result<&mut Self, String> {
        let pool = PgPoolOptions::new()
            .max_connections(max_db_connections)
            .min_connections(2)
            .connect(connection_url)
            .await
            .map_err(|e| format!("Can't create PG connection pool. Error: {e}"))?;

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool, logger));

        let restore_password_token_repository =
            Arc::new(RestorePasswordTokenRepository::new(pg_adapter.clone()));

        let token_generator = Arc::new(StringTokenGenerator::new());

        let u_bdi = &self.user_infrastructure_di.business_dic;

        let pbkdf2 = Arc::new(Pbkdf2::new());
        let password_credential_repository = match password_credential_repository {
            Some(r) => r,
            None => Arc::new(PasswordCredentialRepository::new(
                pg_adapter,
                pbkdf2.clone(),
            )),
        };

        let password_credential_checker = Arc::new(PasswordCredentialChecker::new(
            password_credential_repository.clone(),
            pbkdf2,
        ));

        let password_to_token_exchanger = match password_to_token_exchanger {
            Some(ex) => ex,
            None => Arc::new(DefaultPasswordToTokenExchanger::new(
                u_bdi.authenticated_user_repository.clone(),
                u_bdi.session_repository.clone(),
                password_credential_checker,
                self.id_factory.clone(),
                token_generator.clone(),
            )),
        };

        let member_confirms_email_uc = match member_confirms_email_uc {
            Some(u) => u,
            None => Arc::new(DefaultMemberConfirmsEmailUc::new(
                u_bdi.authenticated_user_repository.clone(),
                self.logger.clone(),
            )),
        };

        self.business_builder.services(
            u_bdi.anonymous_binding_repository.clone(),
            u_bdi.anonymous_user_repository.clone(),
            u_bdi.authenticated_user_repository.clone(),
            u_bdi.session_repository.clone(),
            password_credential_repository,
            password_to_token_exchanger,
            restore_password_token_repository,
            token_generator,
            u_bdi.token_router.clone(),
            u_bdi.member_authenticator.clone(),
            member_confirms_email_uc.clone(),
        );

        Ok(self)
    }

    // pub fn mock(&mut self) -> &mut Self {
    //     self.business_builder.mock();

    //     self
    // }

    pub fn build(&self) -> Result<UserAccountInfrastructureDiC<UniqueId, L>, String> {
        let bdi = self.business_builder.build()?;

        let anonymous_registers_handler =
            Arc::new(AnonymousRegistersHandler::new(bdi.anonymous_registers_uc));

        let dto_to_user_registers_with_password_params =
            Arc::new(DtoToUserRegistersWithPasswordParams::new());

        let user_registers_with_password_handler = Arc::new(UserRegistersWithPasswordHandler::new(
            bdi.user_registers_with_password_uc,
            dto_to_user_registers_with_password_params,
        ));

        let dto_to_user_logins_with_password_params =
            Arc::new(DtoToUserLoginsWithPasswordParams::new());

        let user_logins_with_password_handler = Arc::new(UserLoginsWithPasswordHandler::new(
            bdi.user_logins_with_password_uc,
            dto_to_user_logins_with_password_params,
        ));

        let dto_to_anonymous_with_token_changes_password_params =
            Arc::new(DtoToAnonymousWithTokenChangesPasswordParams::new());

        let anonymous_with_token_changes_password =
            Arc::new(AnonymousWithTokenChangesPasswordHandler::new(
                bdi.anonymous_with_token_changes_password_uc,
                dto_to_anonymous_with_token_changes_password_params,
            ));

        let dto_to_anonymous_requests_restore_token_params =
            Arc::new(DtoToAnonymousRequestsRestoreTokenParams::new());

        let anonymous_requests_restore_token = Arc::new(AnonymousRequestsRestoreTokenHandler::new(
            bdi.anonymous_requests_restore_token_uc,
            self.logger.clone(),
            dto_to_anonymous_requests_restore_token_params,
        ));

        let member_verifies_email_handler = Arc::new(MemberVerifiesEmailHandler::new(
            bdi.member_confirms_email_uc.clone(),
            self.logger.clone(),
        ));

        Ok(UserAccountInfrastructureDiC {
            anonymous_registers_handler,
            user_registers_with_password_handler,
            user_logins_with_password_handler,
            anonymous_with_token_changes_password_handler: anonymous_with_token_changes_password,
            anonymous_requests_restore_token_handler: anonymous_requests_restore_token,
            member_verifies_email_handler,
        })
    }
}
