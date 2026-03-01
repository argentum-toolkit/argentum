use crate::db::dto::GuestBindingDto;
use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::entity::guest_binding::GuestBinding;
use argentum_user_business::repository::guest_binding_repository::{
    GuestBindingRepositoryError, GuestBindingRepositoryTrait,
};
use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;

const TABLE_NAME: &str = "ag_user_guest_binding";

pub struct GuestBindingRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<SqlxPostgresAdapter<L>>,
}

impl<L> GuestBindingRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<SqlxPostgresAdapter<L>>) -> Self {
        Self { adapter }
    }
}

#[async_trait]
impl<L> GuestBindingRepositoryTrait for GuestBindingRepository<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn find_by_member_id(
        &self,
        member_id: Self::Id,
    ) -> Result<Option<GuestBinding<Self::Id>>, GuestBindingRepositoryError> {
        let sql = format!(
            "SELECT member_id, guest_id, created_at FROM {TABLE_NAME} WHERE id = $1 LIMIT 1"
        );

        let id: Uuid = member_id.into();
        let query = sqlx::query_as(&sql).bind(id);

        let result: Result<Option<GuestBindingDto>, DbAdapterError> =
            self.adapter.fetch_one(query).await;

        match result {
            Ok(Some(dto)) => Ok(Some(GuestBinding {
                member_id: dto.member_id.into(),
                guest_id: dto.guest_id.into(),
                created_at: dto.created_at,
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(GuestBindingRepositoryError::Find(Some(Box::new(e)))),
        }
    }

    async fn save(
        &self,
        binding: &GuestBinding<Self::Id>,
    ) -> Result<(), GuestBindingRepositoryError> {
        let member_id: Uuid = binding.member_id.into();
        let guest_id: Uuid = binding.guest_id.into();

        let sql = format!(
            "INSERT INTO {TABLE_NAME} (member_id, guest_id, created_at) VALUES ($1, $2, $3)"
        );
        let query = sqlx::query(&sql)
            .bind(member_id)
            .bind(guest_id)
            .bind(binding.created_at);

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(GuestBindingRepositoryError::Save(Some(Box::new(e)))),
        }
    }
}
