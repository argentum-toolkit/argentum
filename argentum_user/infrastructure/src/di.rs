use crate::db::repository::AnonymousBindingRepository;
use crate::db::repository::AnonymousUserRepository;
use crate::db::repository::AuthenticatedUserRepository;
use crate::db::repository::SessionRepository;
use crate::rest::handler::GetUserHandler;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::di::{UserBusinessDiC, UserBusinessDiCBuilder};
use argentum_user_rest::server::handler::GetUserTrait;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct UserInfrastructureDiC<I>
where
    I: Id + 'static, //todo: not static
{
    pub business_dic: UserBusinessDiC<I>,
    pub get_user_handler: Arc<dyn GetUserTrait<Id = I>>,
}

pub struct UserInfrastructureDiCBuilder {
    pub business_builder: UserBusinessDiCBuilder<UniqueId>,
    pub id_factory: Arc<dyn IdFactory<Id = UniqueId>>,
}

impl UserInfrastructureDiCBuilder {
    pub fn new(id_factory: Arc<dyn IdFactory<Id = UniqueId>>) -> Self {
        Self {
            business_builder: UserBusinessDiCBuilder::new(),
            id_factory,
        }
    }

    pub async fn default_services<L>(
        &mut self,
        connection_url: &str,
        max_db_connections: u32,
        logger: Arc<L>,
    ) -> Result<&mut Self, String>
    where
        L: LoggerTrait + 'static,
    {
        let pool = PgPoolOptions::new()
            .max_connections(max_db_connections)
            .min_connections(2)
            .connect(connection_url)
            .await
            .map_err(|e| format!("Can't create PG connection pool. Err: {e}"))?;

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool, logger));

        self.business_builder
            .anonymous_binding_repository(Arc::new(AnonymousBindingRepository::new(
                pg_adapter.clone(),
                // self.id_factory.clone(),
            )))
            .anonymous_user_repository(Arc::new(AnonymousUserRepository::new(
                pg_adapter.clone(),
                // self.id_factory.clone(),
            )))
            .authenticated_user_repository(Arc::new(AuthenticatedUserRepository::new(
                pg_adapter.clone(),
                // self.id_factory.clone(),
            )))
            .session_repository(Arc::new(SessionRepository::new(
                pg_adapter,
                // self.id_factory.clone(),
            )));

        Ok(self)
    }

    pub fn build(&self) -> Result<UserInfrastructureDiC<UniqueId>, String> {
        let business_dic = self.business_builder.build()?;

        let get_user_handler = Arc::new(GetUserHandler::new(business_dic.get_user_uc.clone()));

        Ok(UserInfrastructureDiC {
            business_dic,
            get_user_handler,
        })
    }
}
