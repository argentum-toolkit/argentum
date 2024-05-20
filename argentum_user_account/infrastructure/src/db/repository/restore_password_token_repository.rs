use crate::db::dto::RestorePasswordTokenDto;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db::adapter::DbAdapterError;
use argentum_standard_infrastructure::db::slqx_postgres::SqlxPostgresAdapter;
use argentum_user_account_business::entity::restore_password_token::RestorePasswordToken;
use argentum_user_account_business::repository::restore_password_token_repository::{
    RestorePasswordTokenRepositoryError, RestorePasswordTokenRepositoryTrait,
};
use futures::executor::block_on;
use sqlx::postgres::PgArguments;
use sqlx::query::QueryAs;
use sqlx::Postgres;
use std::sync::Arc;

pub struct RestorePasswordTokenRepository {
    adapter: Arc<SqlxPostgresAdapter>,
    id_factory: Arc<UniqueIdFactory>,
}

impl RestorePasswordTokenRepository {
    pub fn new(adapter: Arc<SqlxPostgresAdapter>, id_factory: Arc<UniqueIdFactory>) -> Self {
        Self {
            adapter,
            id_factory,
        }
    }

    fn find_one<'q>(
        &'q self,
        // token: String,
        query: QueryAs<'q, Postgres, RestorePasswordTokenDto, PgArguments>,
    ) -> Result<Option<RestorePasswordToken>, RestorePasswordTokenRepositoryError> {
        let result: Result<Option<RestorePasswordTokenDto>, DbAdapterError> =
            block_on(self.adapter.fetch_one(query));

        match result {
            Ok(Some(dto)) => Ok(Some(RestorePasswordToken {
                id: self.id_factory.uuid_to_id(dto.id),
                user_id: self.id_factory.uuid_to_id(dto.user_id),
                token: dto.token,
                created_at: dto.created_at,
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(RestorePasswordTokenRepositoryError::Other(Some(Box::new(
                e,
            )))),
        }
    }
}

impl RestorePasswordTokenRepositoryTrait for RestorePasswordTokenRepository {
    fn find(
        &self,
        token_id: &Id,
    ) -> Result<Option<RestorePasswordToken>, RestorePasswordTokenRepositoryError> {
        let id = self.id_factory.id_to_uuid(&token_id);
        let sql = "SELECT id, user_id, token, created_at FROM ag_user_account_restore_password_token WHERE id = $1 LIMIT 1";
        let query = sqlx::query_as(sql).bind(&id);

        self.find_one(query)
    }

    fn find_by_token(
        &self,
        token: String,
    ) -> Result<Option<RestorePasswordToken>, RestorePasswordTokenRepositoryError> {
        let sql = "SELECT id, user_id, token, created_at FROM ag_user_account_restore_password_token WHERE token = $1 LIMIT 1";
        let query = sqlx::query_as(sql).bind(token);

        self.find_one(query)
    }

    fn save(
        &self,
        token: &RestorePasswordToken,
    ) -> Result<(), RestorePasswordTokenRepositoryError> {
        let id = self.id_factory.id_to_uuid(&token.id);
        let user_id = self.id_factory.id_to_uuid(&token.user_id);

        let sql = "INSERT INTO ag_user_account_restore_password_token (id, user_id, token, created_at) VALUES ($1, $2, $3, $4)";

        let query = sqlx::query(sql)
            .bind(&id)
            .bind(&user_id)
            .bind(token.token.clone())
            .bind(token.created_at.naive_utc());

        let result = block_on(self.adapter.exec(query));

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RestorePasswordTokenRepositoryError::Save(Some(Box::new(e)))),
        }
    }

    fn delete_users_tokens(&self, user_id: &Id) -> Result<(), RestorePasswordTokenRepositoryError> {
        let id = &self.id_factory.id_to_uuid(&user_id);
        let sql = "DELETE FROM ag_user_account_restore_password_token WHERE user_id = $1";
        let query = sqlx::query(sql).bind(id);

        let result = block_on(self.adapter.exec(query));

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RestorePasswordTokenRepositoryError::Delete(Some(Box::new(
                e,
            )))),
        }
    }
}
