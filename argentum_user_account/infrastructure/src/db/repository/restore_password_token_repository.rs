use crate::db::UserAccountSqlxPostgresAdapter;
use crate::db::dto::RestorePasswordTokenDto;
use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::entity::RestorePasswordToken;
use argentum_user_account_business::repository::restore_password_token_repository::{
    RestorePasswordTokenRepositoryError, RestorePasswordTokenRepositoryTrait,
};
use sqlx::Postgres;
use sqlx::postgres::PgArguments;
use sqlx::query::QueryAs;
use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;

const TABLE_NAME: &str = "ag_user_account_restore_password_token";

pub struct RestorePasswordTokenRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<UserAccountSqlxPostgresAdapter<L>>,
}

impl<L> RestorePasswordTokenRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<UserAccountSqlxPostgresAdapter<L>>) -> Self {
        Self { adapter }
    }

    async fn find_one<'q>(
        &'q self,
        query: QueryAs<'q, Postgres, RestorePasswordTokenDto, PgArguments>,
    ) -> Result<Option<RestorePasswordToken<UniqueId>>, RestorePasswordTokenRepositoryError> {
        let result: Result<Option<RestorePasswordTokenDto>, DbAdapterError> =
            self.adapter.fetch_one(query).await;

        match result {
            Ok(Some(dto)) => Ok(Some(RestorePasswordToken {
                id: dto.id.into(),
                user_id: dto.user_id.into(),
                token: dto.token,
                created_at: dto.created_at,
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(RestorePasswordTokenRepositoryError::Find(Some(Arc::new(e)))),
        }
    }
}

#[async_trait]
impl<L> RestorePasswordTokenRepositoryTrait for RestorePasswordTokenRepository<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn find(
        &self,
        token_id: Self::Id,
    ) -> Result<Option<RestorePasswordToken<Self::Id>>, RestorePasswordTokenRepositoryError> {
        let id: Uuid = token_id.into();
        let sql = format!(
            "SELECT id, user_id, token, created_at FROM {TABLE_NAME} WHERE id = $1 LIMIT 1"
        );
        let query = sqlx::query_as(&sql).bind(id);

        self.find_one(query).await
    }

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<RestorePasswordToken<Self::Id>>, RestorePasswordTokenRepositoryError> {
        let sql = format!(
            "SELECT id, user_id, token, created_at FROM {TABLE_NAME} WHERE token = $1 LIMIT 1"
        );
        let query = sqlx::query_as(&sql).bind(token);

        self.find_one(query).await
    }

    async fn save(
        &self,
        token: &RestorePasswordToken<Self::Id>,
    ) -> Result<(), RestorePasswordTokenRepositoryError> {
        let sql = format!(
            "INSERT INTO {TABLE_NAME} (id, user_id, token, created_at) VALUES ($1, $2, $3, $4)"
        );

        let id: Uuid = token.id.into();
        let user_id: Uuid = token.user_id.into();

        let query = sqlx::query(&sql)
            .bind(id)
            .bind(user_id)
            .bind(token.token.clone())
            .bind(token.created_at.naive_utc());

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RestorePasswordTokenRepositoryError::Save(Some(Arc::new(e)))),
        }
    }

    async fn delete_users_tokens(
        &self,
        user_id: Self::Id,
    ) -> Result<(), RestorePasswordTokenRepositoryError> {
        let id: Uuid = user_id.into();
        let sql = format!("DELETE FROM {TABLE_NAME} WHERE user_id = $1");
        let query = sqlx::query(&sql).bind(id);

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RestorePasswordTokenRepositoryError::Delete(Some(Arc::new(
                e,
            )))),
        }
    }
}
