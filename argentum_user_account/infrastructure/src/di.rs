use crate::db::repository::PasswordCredentialRepository;
use crate::db::repository::RestorePasswordTokenRepository;
use crate::rest::handler::{
    AnonymousRegistersHandler, AnonymousRequestsRestoreTokenHandler,
    AnonymousWithTokenChangesPasswordHandler, UserLoginsWithPasswordHandler,
    UserRegistersWithPasswordHandler,
};
use crate::rest::transformer::{
    DtoToAnonymousRequestsRestoreTokenParams, DtoToAnonymousWithTokenChangesPasswordParams,
    DtoToUserLoginsWithPasswordParams, DtoToUserRegistersWithPasswordParams,
};
use crate::token::StringTokenGenerator;
use argentum_encryption_business::password::{Encryptor, Validator};
use argentum_log_business::LoggerTrait;
use argentum_notification_business::NotificatorTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_account_business::di::UserAccountBusinessDiCBuilder;
use argentum_user_account_rest::server::handler::{
    AnonymousRegistersTrait, AnonymousWithTokenChangesPasswordTrait, UserLoginsWithPasswordTrait,
    UserRegistersWithPasswordTrait,
};
use argentum_user_infrastructure::di::UserInfrastructureDiC;
use std::rc::Rc;

use argentum_standard_infrastructure::db::slqx_postgres::SqlxPostgresAdapter;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct UserAccountInfrastructureDiC {
    // Public services
    pub anonymous_registers_handler: Arc<dyn AnonymousRegistersTrait>,
    pub anonymous_requests_restore_token_handler: Arc<AnonymousRequestsRestoreTokenHandler>,
    pub anonymous_with_token_changes_password_handler:
        Arc<dyn AnonymousWithTokenChangesPasswordTrait>,
    pub user_registers_with_password_handler: Arc<dyn UserRegistersWithPasswordTrait>,
    pub user_logins_with_password_handler: Arc<dyn UserLoginsWithPasswordTrait>,
}

pub struct UserAccountInfrastructureDiCBuilder {
    user_infrastructure_di: Rc<UserInfrastructureDiC>,
    business_builder: UserAccountBusinessDiCBuilder,
    id_factory: Arc<UniqueIdFactory>,
    logger: Arc<dyn LoggerTrait>,
}

impl UserAccountInfrastructureDiCBuilder {
    pub fn new(
        user_infrastructure_di: Rc<UserInfrastructureDiC>,
        id_factory: Arc<UniqueIdFactory>,

        encryptor: Arc<dyn Encryptor>,
        validator: Arc<dyn Validator>,
        logger: Arc<dyn LoggerTrait>,
        notificator: Arc<dyn NotificatorTrait>,
    ) -> Self {
        Self {
            user_infrastructure_di,
            business_builder: UserAccountBusinessDiCBuilder::new(
                id_factory.clone(),
                encryptor,
                validator,
                logger.clone(),
                notificator,
            ),
            id_factory,
            logger,
        }
    }

    pub fn config(
        &mut self,
        product_name: String,
        restore_password_token_ttl: u32,
        restore_password_front_url: String,
    ) -> &mut Self {
        self.business_builder.config(
            product_name,
            restore_password_token_ttl,
            restore_password_front_url,
        );

        self
    }

    pub async fn services(
        &mut self,
        id_factory: Arc<UniqueIdFactory>,
        connection_url: &str,
        max_db_connections: u32,
        logger: Arc<dyn LoggerTrait>,
    ) -> &mut Self {
        let pool = Arc::new(
            PgPoolOptions::new()
                .max_connections(max_db_connections)
                .connect(connection_url)
                .await
                .unwrap(),
        );

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool, logger));

        let password_credential_repository = Arc::new(PasswordCredentialRepository::new(
            pg_adapter.clone(),
            id_factory.clone(),
        ));

        let restore_password_token_repository =
            Arc::new(RestorePasswordTokenRepository::new(pg_adapter, id_factory));

        let token_generator = Arc::new(StringTokenGenerator::new());

        let u_bdi = &self.user_infrastructure_di.business_dic;

        self.business_builder.services(
            u_bdi.anonymous_binding_repository.clone(),
            u_bdi.anonymous_user_repository.clone(),
            u_bdi.authenticated_user_repository.clone(),
            u_bdi.session_repository.clone(),
            password_credential_repository,
            restore_password_token_repository,
            token_generator,
        );

        self
    }

    pub fn mock(&mut self) -> &mut Self {
        self.business_builder.mock();

        self
    }

    pub fn build(&self) -> UserAccountInfrastructureDiC {
        let bdi = self.business_builder.build();

        let anonymous_registers_handler = Arc::new(AnonymousRegistersHandler::new(
            bdi.anonymous_registers_uc,
            self.id_factory.clone(),
        ));

        let dto_to_user_registers_with_password_params =
            Arc::new(DtoToUserRegistersWithPasswordParams::new());

        let user_registers_with_password_handler = Arc::new(UserRegistersWithPasswordHandler::new(
            bdi.user_registers_with_password_uc,
            self.id_factory.clone(),
            dto_to_user_registers_with_password_params,
        ));

        let dto_to_user_logins_with_password_params =
            Arc::new(DtoToUserLoginsWithPasswordParams::new());

        let user_logins_with_password_handler = Arc::new(UserLoginsWithPasswordHandler::new(
            bdi.user_logins_with_password_uc,
            self.id_factory.clone(),
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

        UserAccountInfrastructureDiC {
            anonymous_registers_handler,
            user_registers_with_password_handler,
            user_logins_with_password_handler,
            anonymous_with_token_changes_password_handler: anonymous_with_token_changes_password,
            anonymous_requests_restore_token_handler: anonymous_requests_restore_token,
        }
    }
}
