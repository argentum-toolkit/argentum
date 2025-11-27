use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_encryption_business::password::Encryptor;
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::entity::credential::{
    EncodedPasswordCredential, PasswordCredential,
};
use argentum_user_account_business::repository::password_credential_repository::{
    PasswordCredentialRepositoryError, PasswordCredentialRepositoryTrait,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use secrecy::ExposeSecret;

use crate::db::dto::PasswordCredentialDto;

const TABLE_NAME: &str = "ag_user_account_password_credential";

pub struct PasswordCredentialRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<SqlxPostgresAdapter<L>>,
    encryptor: Arc<dyn Encryptor>,
}

impl<L> PasswordCredentialRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<SqlxPostgresAdapter<L>>, encryptor: Arc<dyn Encryptor>) -> Self {
        Self { adapter, encryptor }
    }
}

#[async_trait]
impl<L> PasswordCredentialRepositoryTrait for PasswordCredentialRepository<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn save(
        &self,
        cred: &PasswordCredential<Self::Id>,
    ) -> Result<(), PasswordCredentialRepositoryError> {
        let (password, salt) = self.encryptor.encrypt(&cred.password)?;

        let user_id: Uuid = cred.user_id.into();
        let sql = format!("INSERT INTO {TABLE_NAME} (user_id, password, salt) VALUES ($1, $2, $3)");
        let query = sqlx::query(&sql)
            .bind(user_id)
            .bind(password.expose_secret())
            .bind(salt.expose_secret());

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PasswordCredentialRepositoryError::Save(Arc::new(e))),
        }
    }

    async fn find_by_user_id(
        &self,
        user_id: Self::Id,
    ) -> Result<Option<EncodedPasswordCredential<Self::Id>>, PasswordCredentialRepositoryError>
    {
        let id: Uuid = user_id.into();
        let sql =
            format!("SELECT user_id, password, salt FROM {TABLE_NAME} WHERE user_id = $1 LIMIT 1");
        let query_as = sqlx::query_as(&sql).bind(id);
        let result: Result<Option<PasswordCredentialDto>, DbAdapterError> =
            self.adapter.fetch_one(query_as).await;

        match result {
            Ok(Some(dto)) => Ok(Some(EncodedPasswordCredential::new(
                dto.user_id.into(),
                dto.password.into(),
                dto.salt.into(),
            ))),
            Ok(None) => Ok(None),
            Err(e) => Err(PasswordCredentialRepositoryError::Find(Arc::new(e))),
        }
    }

    async fn delete(
        &self,
        cred: EncodedPasswordCredential<Self::Id>,
    ) -> Result<(), PasswordCredentialRepositoryError> {
        let user_id: Uuid = cred.user_id.into();
        let sql = format!("DELETE FROM {TABLE_NAME} WHERE user_id = $1");

        let query = sqlx::query(&sql).bind(user_id);

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PasswordCredentialRepositoryError::Delete(Arc::new(e))),
        }
    }
}
