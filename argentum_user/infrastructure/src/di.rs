use crate::db::repository::AnonymousBindingRepository;
use crate::db::repository::AnonymousUserRepository;
use crate::db::repository::AuthenticatedUserRepository;
use crate::db::repository::SessionRepository;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db::slqx_postgres::SqlxPostgresAdapter;
use argentum_user_business::di::{UserBusinessDiC, UserBusinessDiCBuilder};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct UserInfrastructureDiC {
    pub business_dic: UserBusinessDiC,
}

#[derive(Default)]
pub struct UserDiCBuilder {
    pub business_dic_builder: UserBusinessDiCBuilder,
}

impl UserDiCBuilder {
    pub fn new() -> Self {
        Self {
            business_dic_builder: UserBusinessDiCBuilder::default(),
        }
    }

    pub async fn default_services(
        &mut self,
        connection_url: &str,
        max_db_connections: u32,
        id_factory: Arc<UniqueIdFactory>,
    ) -> &mut Self {
        let pool = Arc::new(
            PgPoolOptions::new()
                .max_connections(max_db_connections)
                .connect(connection_url)
                .await
                .unwrap(),
        );

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool));

        self.business_dic_builder
            .anonymous_binding_repository(Arc::new(AnonymousBindingRepository::new(
                pg_adapter.clone(),
                id_factory.clone(),
            )))
            .anonymous_user_repository(Arc::new(AnonymousUserRepository::new(
                pg_adapter.clone(),
                id_factory.clone(),
            )))
            .authenticated_user_repository(Arc::new(AuthenticatedUserRepository::new(
                pg_adapter.clone(),
                id_factory.clone(),
            )))
            .session_repository(Arc::new(SessionRepository::new(pg_adapter, id_factory)));

        self
    }

    pub fn build(&self) -> UserInfrastructureDiC {
        UserInfrastructureDiC {
            business_dic: self.business_dic_builder.build(),
        }
    }
}
