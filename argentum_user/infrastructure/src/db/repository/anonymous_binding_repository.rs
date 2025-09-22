use crate::db::dto::AnonymousBindingDto;
use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::entity::anonymous_binding::AnonymousBinding;
use argentum_user_business::repository::anonymous_binding_repository::{
    AnonymousBindingRepositoryError, AnonymousBindingRepositoryTrait,
};
use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;

const TABLE_NAME: &str = "ag_user_anonymous_binding";

pub struct AnonymousBindingRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<SqlxPostgresAdapter<L>>,
}

impl<L> AnonymousBindingRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<SqlxPostgresAdapter<L>>) -> Self {
        Self { adapter }
    }
}

#[async_trait]
impl<L> AnonymousBindingRepositoryTrait for AnonymousBindingRepository<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn find_by_user_id(
        &self,
        user_id: Self::Id,
    ) -> Result<Option<AnonymousBinding<Self::Id>>, AnonymousBindingRepositoryError> {
        let sql = format!(
            "SELECT user_id, anonymous_id, created_at FROM {TABLE_NAME} WHERE id = $1 LIMIT 1"
        );

        let id: Uuid = user_id.into();
        let query = sqlx::query_as(&sql).bind(id);

        let result: Result<Option<AnonymousBindingDto>, DbAdapterError> =
            self.adapter.fetch_one(query).await;

        match result {
            Ok(Some(dto)) => Ok(Some(AnonymousBinding {
                user_id: dto.user_id.into(),
                anonymous_id: dto.anonymous_id.into(),
                created_at: dto.created_at,
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(AnonymousBindingRepositoryError::Find(Some(Box::new(e)))),
        }
    }

    async fn save(
        &self,
        binding: &AnonymousBinding<Self::Id>,
    ) -> Result<(), AnonymousBindingRepositoryError> {
        let user_id: Uuid = binding.user_id.into();
        let anonymous_id: Uuid = binding.anonymous_id.into();

        let sql = format!(
            "INSERT INTO {TABLE_NAME} (user_id, anonymous_id, created_at) VALUES ($1, $2, $3)"
        );
        let query = sqlx::query(&sql)
            .bind(user_id)
            .bind(anonymous_id)
            .bind(binding.created_at);

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(AnonymousBindingRepositoryError::Save(Some(Box::new(e)))),
        }
    }
}
