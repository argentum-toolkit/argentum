use crate::db::dto::AnonymousUserDto;
use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::entity::user::AnonymousUser;
use argentum_user_business::repository::user_repository::{
    AnonymousUserRepositoryTrait, ExternalUserError,
};
use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;

const TABLE_NAME: &str = "ag_user_anonymous";

pub struct AnonymousUserRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<SqlxPostgresAdapter<L>>,
}

impl<L> AnonymousUserRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<SqlxPostgresAdapter<L>>) -> Self {
        Self { adapter }
    }
}

#[async_trait]
impl<L> AnonymousUserRepositoryTrait for AnonymousUserRepository<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;
    async fn find(
        &self,
        id: Self::Id,
    ) -> Result<Option<AnonymousUser<Self::Id>>, ExternalUserError> {
        let anonymous_id: Uuid = id.into();
        let sql = format!("SELECT id, created_at FROM {TABLE_NAME} WHERE id = $1 LIMIT 1");
        let query = sqlx::query_as(&sql).bind(anonymous_id);

        let result: Result<Option<AnonymousUserDto>, DbAdapterError> =
            self.adapter.fetch_one(query).await;

        match result {
            Ok(Some(dto)) => Ok(Some(AnonymousUser {
                id: dto.id.into(),
                created_at: dto.created_at,
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(ExternalUserError::Anonymous(Some(Arc::new(e)))),
        }
    }

    async fn save(&self, user: &AnonymousUser<Self::Id>) -> Result<(), ExternalUserError> {
        let sql = format!("INSERT INTO {TABLE_NAME} (id, created_at) VALUES ($1, $2)");
        let user_id: Uuid = user.id.into();
        let query = sqlx::query(&sql).bind(user_id).bind(user.created_at);

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ExternalUserError::Anonymous(Some(Arc::new(e)))),
        }
    }
}
