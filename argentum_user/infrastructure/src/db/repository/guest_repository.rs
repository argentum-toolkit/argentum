use crate::db::UserSqlxPostgresAdapter;
use crate::db::dto::GuestDto;
use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::entity::user::Guest;
use argentum_user_business::repository::user_repository::{
    ExternalUserError, GuestUserRepositoryTrait,
};
use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;

const TABLE_NAME: &str = "ag_user_guest";

pub struct GuestRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<UserSqlxPostgresAdapter<L>>,
}

impl<L> GuestRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<UserSqlxPostgresAdapter<L>>) -> Self {
        Self { adapter }
    }
}

#[async_trait]
impl<L> GuestUserRepositoryTrait for GuestRepository<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;
    async fn find(&self, id: Self::Id) -> Result<Option<Guest<Self::Id>>, ExternalUserError> {
        let guest_id: Uuid = id.into();
        let sql = format!("SELECT id, created_at FROM {TABLE_NAME} WHERE id = $1 LIMIT 1");
        let query = sqlx::query_as(&sql).bind(guest_id);

        let result: Result<Option<GuestDto>, DbAdapterError> = self.adapter.fetch_one(query).await;

        match result {
            Ok(Some(dto)) => Ok(Some(Guest {
                id: dto.id.into(),
                created_at: dto.created_at,
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(ExternalUserError::Guest(Some(Arc::new(e)))),
        }
    }

    async fn save(&self, user: &Guest<Self::Id>) -> Result<(), ExternalUserError> {
        let sql = format!("INSERT INTO {TABLE_NAME} (id, created_at) VALUES ($1, $2)");
        let user_id: Uuid = user.id.into();
        let query = sqlx::query(&sql).bind(user_id).bind(user.created_at);

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ExternalUserError::Guest(Some(Arc::new(e)))),
        }
    }
}
