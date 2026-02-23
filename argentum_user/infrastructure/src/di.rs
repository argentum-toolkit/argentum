use crate::db::repository::AnonymousBindingRepository;
use crate::db::repository::AnonymousUserRepository;
use crate::db::repository::AuthenticatedUserRepository;
use crate::db::repository::SessionRepository;
use crate::rest::handler::GetUserHandler;
use crate::service::MemberAuthenticator;
use crate::service::TokenRouter;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::di::RestDiBuilder;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::di::{UserBusinessDi, UserBusinessDiBuilder};
use argentum_user_business::repository::user_repository::AuthenticatedUserRepositoryTrait;
use argentum_user_business::service::MemberAuthenticatorTrait;
use argentum_user_rest::ApiDiC;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::sync::Mutex;

pub struct UserInfrastructureDi<I, L>
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    pub business_di: UserBusinessDi<I, L>,
    pub api_di: ApiDiC<I>,
}

pub struct UserInfrastructureDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    connection_url: String,
    max_db_connections: u32,
    business_builder: Arc<Mutex<UserBusinessDiBuilder<UniqueId, L>>>,
    rest_di: Arc<RestDiBuilder<UniqueId, L>>,
    logger: Arc<L>,
}

impl<L> UserInfrastructureDiBuilder<L>
where
    L: LoggerTrait,
{
    pub fn new(
        connection_url: &str,
        max_db_connections: u32,

        business_builder: Arc<Mutex<UserBusinessDiBuilder<UniqueId, L>>>,
        rest_builder: Arc<RestDiBuilder<UniqueId, L>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            connection_url: connection_url.into(),
            max_db_connections,
            business_builder,
            rest_di: rest_builder,
            logger,
        }
    }

    pub async fn with_default_services(&mut self) -> Result<&mut Self, String> {
        let pool = PgPoolOptions::new()
            .max_connections(self.max_db_connections)
            .min_connections(2)
            .connect(&self.connection_url)
            .await
            .map_err(|e| format!("Can't create PG connection pool. Err: {e}"))?;

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool, self.logger.clone()));

        let session_repository = Arc::new(SessionRepository::new(pg_adapter.clone()));
        self.business_builder
            .lock()
            .unwrap()
            .anonymous_binding_repository(Arc::new(AnonymousBindingRepository::new(
                pg_adapter.clone(),
            )))
            .anonymous_user_repository(Arc::new(AnonymousUserRepository::new(pg_adapter.clone())))
            .session_repository(session_repository.clone())
            .token_router(Arc::new(TokenRouter::new()));

        let member_repo = Arc::new(AuthenticatedUserRepository::new(pg_adapter.clone()));

        self.business_builder
            .lock()
            .unwrap()
            .authenticated_user_repository(member_repo.clone());

        self.business_builder
            .lock()
            .unwrap()
            .member_authenticator(Arc::new(MemberAuthenticator::new(
                member_repo.clone(),
                session_repository,
            )));

        Ok(self)
    }

    pub async fn with_member_repository(
        &mut self,
        member_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = UniqueId>>,
    ) -> Result<&mut Self, String> {
        self.business_builder
            .lock()
            .unwrap()
            .authenticated_user_repository(member_repository.clone());

        Ok(self)
    }

    pub async fn with_member_authenticator(
        &mut self,
        member_authenticator: Arc<dyn MemberAuthenticatorTrait<Id = UniqueId>>,
    ) -> Result<&mut Self, String> {
        self.business_builder
            .lock()
            .unwrap()
            .member_authenticator(member_authenticator);

        Ok(self)
    }

    pub fn build(&self) -> Result<UserInfrastructureDi<UniqueId, L>, String> {
        let business_di = self.business_builder.lock().unwrap().build()?;

        let get_user_handler = Arc::new(GetUserHandler::new(business_di.get_user_uc.clone()));

        let rest_di = self.rest_di.build();
        let api_di = ApiDiC::new(
            "/api/v1".to_string(),
            rest_di.request_transformer.clone(),
            rest_di.bearer_authenticator.clone(),
            get_user_handler.clone(),
            rest_di.error_pre_handler.clone(),
        );

        Ok(UserInfrastructureDi {
            business_di,
            api_di,
        })
    }
}
