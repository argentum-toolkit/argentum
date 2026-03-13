use crate::db::dto::EmailConfirmationTokenDto;
use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::entity::EmailConfirmationToken;
use argentum_user_account_business::repository::email_confirmation_token_repository::{
    EmailConfirmationTokenRepositoryError, EmailConfirmationTokenRepositoryTrait,
};
use sqlx::Postgres;
use sqlx::postgres::PgArguments;
use sqlx::query::QueryAs;
use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;

const TABLE_NAME: &str = "ag_user_account_email_confirmation_token";

pub struct EmailConfirmationTokenRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<SqlxPostgresAdapter<L>>,
}

impl<L> EmailConfirmationTokenRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<SqlxPostgresAdapter<L>>) -> Self {
        Self { adapter }
    }

    async fn find_one<'q>(
        &'q self,
        query: QueryAs<'q, Postgres, EmailConfirmationTokenDto, PgArguments>,
    ) -> Result<Option<EmailConfirmationToken<UniqueId>>, EmailConfirmationTokenRepositoryError>
    {
        let result: Result<Option<EmailConfirmationTokenDto>, DbAdapterError> =
            self.adapter.fetch_one(query).await;

        match result {
            Ok(Some(dto)) => Ok(Some(EmailConfirmationToken {
                id: dto.id.into(),
                user_id: dto.user_id.into(),
                token: dto.token,
                created_at: dto.created_at,
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(EmailConfirmationTokenRepositoryError::Find(Some(Arc::new(
                e,
            )))),
        }
    }
}

#[async_trait]
impl<L> EmailConfirmationTokenRepositoryTrait for EmailConfirmationTokenRepository<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn find(
        &self,
        token_id: Self::Id,
    ) -> Result<Option<EmailConfirmationToken<Self::Id>>, EmailConfirmationTokenRepositoryError>
    {
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
    ) -> Result<Option<EmailConfirmationToken<Self::Id>>, EmailConfirmationTokenRepositoryError>
    {
        let sql = format!(
            "SELECT id, user_id, token, created_at FROM {TABLE_NAME} WHERE token = $1 LIMIT 1"
        );
        let query = sqlx::query_as(&sql).bind(token);

        self.find_one(query).await
    }

    async fn save(
        &self,
        token: &EmailConfirmationToken<Self::Id>,
    ) -> Result<(), EmailConfirmationTokenRepositoryError> {
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
            Err(e) => Err(EmailConfirmationTokenRepositoryError::Save(Some(Arc::new(
                e,
            )))),
        }
    }

    async fn delete_users_tokens(
        &self,
        user_id: Self::Id,
    ) -> Result<(), EmailConfirmationTokenRepositoryError> {
        let id: Uuid = user_id.into();
        let sql = format!("DELETE FROM {TABLE_NAME} WHERE user_id = $1");
        let query = sqlx::query(&sql).bind(id);

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(EmailConfirmationTokenRepositoryError::Delete(Some(
                Arc::new(e),
            ))),
        }
    }
}
