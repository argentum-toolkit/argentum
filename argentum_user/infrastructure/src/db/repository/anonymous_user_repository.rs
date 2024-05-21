use crate::db::dto::AnonymousUserDto;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db::adapter::DbAdapterError;
use argentum_standard_infrastructure::db::slqx_postgres::SqlxPostgresAdapter;
use argentum_user_business::entity::user::AnonymousUser;
use argentum_user_business::repository::user_repository::{
    AnonymousUserRepositoryTrait, ExternalUserError,
};
use futures::executor::block_on;
use std::sync::Arc;

pub struct AnonymousUserRepository {
    adapter: Arc<SqlxPostgresAdapter>,
    id_factory: Arc<UniqueIdFactory>,
}

impl AnonymousUserRepository {
    pub fn new(adapter: Arc<SqlxPostgresAdapter>, id_factory: Arc<UniqueIdFactory>) -> Self {
        Self {
            adapter,
            id_factory,
        }
    }
}

impl AnonymousUserRepositoryTrait for AnonymousUserRepository {
    fn find(&self, id: &Id) -> Result<Option<AnonymousUser>, ExternalUserError> {
        let user_id = self.id_factory.id_to_uuid(&id);
        //move todo table name/prefix to const/param
        let sql = "SELECT id, created_at FROM ag_user_anonymous WHERE id = $1 LIMIT 1";
        let query = sqlx::query_as(sql).bind(user_id);

        let result: Result<Option<AnonymousUserDto>, DbAdapterError> =
            block_on(self.adapter.fetch_one(query));

        match result {
            Ok(Some(dto)) => Ok(Some(AnonymousUser {
                id: self.id_factory.uuid_to_id(dto.id),
                created_at: dto.created_at.clone(),
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(ExternalUserError::Anonymous(Some(Box::new(e)))),
        }
    }

    fn save(&self, user: &AnonymousUser) -> Result<(), ExternalUserError> {
        let id = self.id_factory.id_to_uuid(&user.id);

        let sql = "INSERT INTO ag_user_anonymous (id, created_at) VALUES ($1, $2)";
        let query = sqlx::query(sql).bind(&id).bind(user.created_at);

        let result = block_on(self.adapter.exec(query));

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ExternalUserError::Anonymous(Some(Box::new(e)))),
        }
    }
}
