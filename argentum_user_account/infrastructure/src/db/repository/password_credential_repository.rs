use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_account_business::entity::credential::PasswordCredential;
use argentum_user_account_business::repository::password_credential_repository::{
    PasswordCredentialRepositoryError, PasswordCredentialRepositoryTrait,
};
use std::sync::Arc;

use crate::db::dto::PasswordCredentialDto;
use futures::executor::block_on;

const TABLE_NAME: &str = "ag_user_account_password_credential";

pub struct PasswordCredentialRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<SqlxPostgresAdapter<L>>,
    id_factory: Arc<UniqueIdFactory>,
}

impl<L> PasswordCredentialRepository<L>
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

impl<L> PasswordCredentialRepositoryTrait for PasswordCredentialRepository<L>
where
    L: LoggerTrait,
{
    fn save(&self, cred: &PasswordCredential) -> Result<(), PasswordCredentialRepositoryError> {
        let user_id = self.id_factory.id_to_uuid(&cred.user_id);
        let sql = format!("INSERT INTO {TABLE_NAME} (user_id, password, salt) VALUES ($1, $2, $3)");
        let query = sqlx::query(&sql)
            .bind(user_id)
            .bind(cred.password.clone())
            .bind(cred.salt.clone());

        let result = block_on(self.adapter.exec(query));

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PasswordCredentialRepositoryError::Save(Box::new(e))),
        }
    }

    fn find_by_user_id(
        &self,
        user_id: &Id,
    ) -> Result<Option<PasswordCredential>, PasswordCredentialRepositoryError> {
        let id = self.id_factory.id_to_uuid(user_id);

        let sql =
            format!("SELECT user_id, password, salt FROM {TABLE_NAME} WHERE user_id = $1 LIMIT 1");
        let query_as = sqlx::query_as(&sql).bind(id);
        let result: Result<Option<PasswordCredentialDto>, DbAdapterError> =
            block_on(self.adapter.fetch_one(query_as));

        match result {
            Ok(Some(dto)) => Ok(Some(PasswordCredential::new(
                self.id_factory.uuid_to_id(dto.user_id),
                dto.password,
                dto.salt,
            ))),
            Ok(None) => Ok(None),
            Err(e) => Err(PasswordCredentialRepositoryError::Find(Box::new(e))),
        }
    }

    fn delete(&self, cred: &PasswordCredential) -> Result<(), PasswordCredentialRepositoryError> {
        let user_id = self.id_factory.id_to_uuid(&cred.user_id);

        let sql = format!("DELETE FROM {TABLE_NAME} WHERE user_id = $1");

        let query = sqlx::query(&sql).bind(user_id);

        let result = block_on(self.adapter.exec(query));

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PasswordCredentialRepositoryError::Delete(Box::new(e))),
        }
    }
}
