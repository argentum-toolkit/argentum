use crate::db::repository::AnonymousBindingRepository;
use crate::db::repository::AnonymousUserRepository;
use crate::db::repository::AuthenticatedUserRepository;
use crate::db::repository::SessionRepository;
use crate::rest::handler::GetUserHandler;
use crate::service::MemberAuthenticator;
use crate::service::TokenRouter;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::di::{UserBusinessDiC, UserBusinessDiCBuilder};
use argentum_user_business::repository::user_repository::AuthenticatedUserRepositoryTrait;
use argentum_user_business::service::MemberAuthenticatorTrait;
use argentum_user_rest::server::handler::GetUserTrait;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct UserInfrastructureDiC<I, L>
where
    I: Id + 'static, //todo: not static
    L: LoggerTrait + 'static,
{
    pub business_dic: UserBusinessDiC<I, L>,
    pub get_user_handler: Arc<dyn GetUserTrait<Id = I>>,
}

pub struct UserInfrastructureDiCBuilder<L>
where
    L: LoggerTrait + 'static,
{
    pub business_builder: UserBusinessDiCBuilder<UniqueId, L>,
    pub id_factory: Arc<dyn IdFactory<Id = UniqueId>>,
    logger: Arc<L>,
}

impl<L> UserInfrastructureDiCBuilder<L>
where
    L: LoggerTrait,
{
    pub fn new(id_factory: Arc<dyn IdFactory<Id = UniqueId>>, logger: Arc<L>) -> Self {
        Self {
            business_builder: UserBusinessDiCBuilder::new(logger.clone()),
            id_factory,
            logger,
        }
    }

    pub async fn default_services(
        &mut self,
        connection_url: &str,
        max_db_connections: u32,
        authenticated_user_repository: Option<
            Arc<dyn AuthenticatedUserRepositoryTrait<Id = UniqueId>>,
        >,
        // client: Arc<reqwest::Client>,
        member_authenticator: Option<Arc<dyn MemberAuthenticatorTrait<Id = UniqueId>>>,
    ) -> Result<&mut Self, String> {
        let pool = PgPoolOptions::new()
            .max_connections(max_db_connections)
            .min_connections(2)
            .connect(connection_url)
            .await
            .map_err(|e| format!("Can't create PG connection pool. Err: {e}"))?;

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool, self.logger.clone()));

        let session_repository = Arc::new(SessionRepository::new(pg_adapter.clone()));
        self.business_builder
            .anonymous_binding_repository(Arc::new(AnonymousBindingRepository::new(
                pg_adapter.clone(),
            )))
            .anonymous_user_repository(Arc::new(AnonymousUserRepository::new(pg_adapter.clone())))
            // .authenticated_user_repository(authenticated_user_repository)
            .session_repository(session_repository.clone())
            .token_router(Arc::new(TokenRouter::new()));

        let member_repo = match authenticated_user_repository {
            Some(r) => r,
            None => Arc::new(AuthenticatedUserRepository::new(
                pg_adapter.clone(),
                // self.id_factory.clone(),
            )),
        };

        self.business_builder
            .authenticated_user_repository(member_repo.clone());

        match member_authenticator {
            Some(m) => self.business_builder.member_authenticator(m),
            None => self
                .business_builder
                .member_authenticator(Arc::new(MemberAuthenticator::new(
                    member_repo.clone(),
                    session_repository,
                    // self.id_factory.clone(),
                ))),
        };

        Ok(self)
    }

    pub fn build(&self) -> Result<UserInfrastructureDiC<UniqueId, L>, String> {
        let business_dic = self.business_builder.build()?;

        let get_user_handler = Arc::new(GetUserHandler::new(business_dic.get_user_uc.clone()));

        Ok(UserInfrastructureDiC {
            business_dic,
            get_user_handler,
        })
    }
}
