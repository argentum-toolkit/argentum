use crate::db::dto::SessionDto;
use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::entity::session::Session;
use argentum_user_business::repository::session_repository::{
    SessionRepositoryError, SessionRepositoryTrait,
};
use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;

const TABLE_NAME: &str = "ag_user_session";

pub struct SessionRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<SqlxPostgresAdapter<L>>,
}

impl<L> SessionRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<SqlxPostgresAdapter<L>>) -> Self {
        Self { adapter }
    }
}

#[async_trait]
impl<L> SessionRepositoryTrait for SessionRepository<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<Session<Self::Id>>, SessionRepositoryError> {
        let sql = format!("SELECT id, user_id, token FROM {TABLE_NAME} WHERE token = $1 LIMIT 1");
        let query = sqlx::query_as(&sql).bind(token);

        let result: Result<Option<SessionDto>, DbAdapterError> =
            self.adapter.fetch_one(query).await;

        match result {
            Ok(Some(dto)) => Ok(Some(Session::<Self::Id> {
                id: dto.id.into(),
                user_id: dto.user_id.into(),
                token: dto.token,
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(SessionRepositoryError::Other(Some(Arc::new(e)))),
        }
    }

    async fn save(&self, session: &Session<Self::Id>) -> Result<(), SessionRepositoryError> {
        let session_id: Uuid = session.id.into();
        let user_id: Uuid = session.user_id.into();

        let sql = format!("INSERT INTO {TABLE_NAME} (id, user_id, token) VALUES ($1, $2, $3)");
        let query = sqlx::query(&sql)
            .bind(session_id)
            .bind(user_id)
            .bind(session.token.clone());

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(SessionRepositoryError::Save(Some(Arc::new(e)))),
        }
    }

    async fn delete_users_sessions(&self, user_id: Self::Id) -> Result<(), SessionRepositoryError> {
        let id: Uuid = user_id.into();
        let sql = format!("DELETE FROM {TABLE_NAME} WHERE user_id = $1");

        let query = sqlx::query(&sql).bind(id);

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(SessionRepositoryError::Delete(Some(Arc::new(e)))),
        }
    }
}
