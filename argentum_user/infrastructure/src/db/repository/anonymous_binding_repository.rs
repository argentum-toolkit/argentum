use crate::db::dto::AnonymousBindingDto;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db::adapter::DbAdapterError;
use argentum_standard_infrastructure::db::slqx_postgres::SqlxPostgresAdapter;
use argentum_user_business::entity::anonymous_binding::AnonymousBinding;
use argentum_user_business::repository::anonymous_binding_repository::{
    AnonymousBindingRepositoryError, AnonymousBindingRepositoryTrait,
};
use futures::executor::block_on;
use std::sync::Arc;

pub struct AnonymousBindingRepository {
    adapter: Arc<SqlxPostgresAdapter>,
    id_factory: Arc<UniqueIdFactory>,
}

impl AnonymousBindingRepository {
    pub fn new(adapter: Arc<SqlxPostgresAdapter>, id_factory: Arc<UniqueIdFactory>) -> Self {
        Self {
            adapter,
            id_factory,
        }
    }
}

impl AnonymousBindingRepositoryTrait for AnonymousBindingRepository {
    fn find_by_user_id(
        &self,
        user_id: &Id,
    ) -> Result<Option<AnonymousBinding>, AnonymousBindingRepositoryError> {
        let id = self.id_factory.id_to_uuid(user_id);
        //move todo table name/prefix to const/param
        let sql = "SELECT user_id, anonymous_id, created_at FROM ag_user_anonymous_binding WHERE id = $1 LIMIT 1";
        let query = sqlx::query_as(sql).bind(id);

        let result: Result<Option<AnonymousBindingDto>, DbAdapterError> =
            block_on(self.adapter.fetch_one(query));

        match result {
            Ok(Some(dto)) => Ok(Some(AnonymousBinding {
                user_id: self.id_factory.uuid_to_id(dto.user_id),
                anonymous_id: self.id_factory.uuid_to_id(dto.anonymous_id),
                created_at: dto.created_at,
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(AnonymousBindingRepositoryError::Find(Some(Box::new(e)))),
        }
    }

    fn save(&self, binding: &AnonymousBinding) -> Result<(), AnonymousBindingRepositoryError> {
        let user_id = self.id_factory.id_to_uuid(&binding.user_id);
        let anonymous_id = self.id_factory.id_to_uuid(&binding.anonymous_id);

        let sql = "INSERT INTO ag_user_anonymous_binding (user_id, anonymous_id, created_at) VALUES ($1, $2, $3)";
        let query = sqlx::query(sql)
            .bind(user_id)
            .bind(anonymous_id)
            .bind(binding.created_at);

        let result = block_on(self.adapter.exec(query));

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(AnonymousBindingRepositoryError::Save(Some(Box::new(e)))),
        }
    }
}
