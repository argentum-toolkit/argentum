use crate::db::repository::AnonymousBindingRepository;
use crate::db::repository::AnonymousUserRepository;
use crate::db::repository::AuthenticatedUserRepository;
use crate::db::repository::SessionRepository;
use crate::rest::handler::GetUserHandler;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_business::di::{UserBusinessDiC, UserBusinessDiCBuilder};
use argentum_user_rest::server::handler::GetUserTrait;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct UserInfrastructureDiC {
    pub business_dic: UserBusinessDiC,
    pub get_user_handler: Arc<dyn GetUserTrait>,
}

#[derive(Default)]
pub struct UserInfrastructureDiCBuilder {
    pub business_builder: UserBusinessDiCBuilder,
    pub id_factory: Arc<UniqueIdFactory>,
}

impl UserInfrastructureDiCBuilder {
    pub fn new(id_factory: Arc<UniqueIdFactory>) -> Self {
        Self {
            business_builder: UserBusinessDiCBuilder::default(),
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
        let pool = Arc::new(
            PgPoolOptions::new()
                .max_connections(max_db_connections)
                .connect(connection_url)
                .await
                .map_err(|e| format!("Can't create PG connecion pool. Err: {e}"))?,
        );

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool, logger));

        self.business_builder
            .anonymous_binding_repository(Arc::new(AnonymousBindingRepository::new(
                pg_adapter.clone(),
                self.id_factory.clone(),
            )))
            .anonymous_user_repository(Arc::new(AnonymousUserRepository::new(
                pg_adapter.clone(),
                self.id_factory.clone(),
            )))
            .authenticated_user_repository(Arc::new(AuthenticatedUserRepository::new(
                pg_adapter.clone(),
                self.id_factory.clone(),
            )))
            .session_repository(Arc::new(SessionRepository::new(
                pg_adapter,
                self.id_factory.clone(),
            )));

        Ok(self)
    }

    pub fn build(&self) -> Result<UserInfrastructureDiC, String> {
        let business_dic = self.business_builder.build()?;

        let get_user_handler = Arc::new(GetUserHandler::new(
            business_dic.get_user_uc.clone(),
            self.id_factory.clone(),
        ));

        Ok(UserInfrastructureDiC {
            business_dic,
            get_user_handler,
        })
    }
}
