use crate::db::dto::SessionDto;
use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_business::entity::session::Session;
use argentum_user_business::repository::session_repository::{
    SessionRepositoryError, SessionRepositoryTrait,
};
use futures::executor::block_on;
use std::sync::Arc;

const TABLE_NAME: &'static str = "ag_user_session";

pub struct SessionRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<SqlxPostgresAdapter<L>>,
    id_factory: Arc<UniqueIdFactory>,
}

impl<L> SessionRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<SqlxPostgresAdapter<L>>, id_factory: Arc<UniqueIdFactory>) -> Self {
        Self {
            adapter,
            id_factory,
        }
    }
}

impl<L> SessionRepositoryTrait for SessionRepository<L>
where
    L: LoggerTrait,
{
    fn find_by_token(&self, token: &str) -> Result<Option<Session>, SessionRepositoryError> {
        //move todo table name/prefix to const/param
        let sql = format!("SELECT id, user_id, token FROM {TABLE_NAME} WHERE token = $1 LIMIT 1");
        let query = sqlx::query_as(&sql).bind(token);

        let result: Result<Option<SessionDto>, DbAdapterError> =
            block_on(self.adapter.fetch_one(query));

        match result {
            Ok(Some(dto)) => Ok(Some(Session {
                id: self.id_factory.uuid_to_id(dto.id),
                user_id: self.id_factory.uuid_to_id(dto.user_id),
                token: dto.token,
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(SessionRepositoryError::Other(Some(Box::new(e)))),
        }
    }

    fn save(&self, session: &Session) -> Result<(), SessionRepositoryError> {
        let id = self.id_factory.id_to_uuid(&session.id);
        let user_id = self.id_factory.id_to_uuid(&session.user_id);

        let sql = format!("INSERT INTO {TABLE_NAME} (id, user_id, token) VALUES ($1, $2, $3)");
        let query = sqlx::query(&sql)
            .bind(id)
            .bind(user_id)
            .bind(session.token.clone());

        let result = block_on(self.adapter.exec(query));

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(SessionRepositoryError::Save(Some(Box::new(e)))),
        }
    }

    fn delete_users_sessions(&self, user_id: &Id) -> Result<(), SessionRepositoryError> {
        let id = self.id_factory.id_to_uuid(user_id);
        let sql = format!("DELETE FROM {TABLE_NAME} WHERE user_id = $1");

        let query = sqlx::query(&sql).bind(id);

        let result = block_on(self.adapter.exec(query));

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(SessionRepositoryError::Delete(Some(Box::new(e)))),
        }
    }
}
