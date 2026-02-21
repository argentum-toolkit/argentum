use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_oidc_business::{
    entity::Credential,
    repository::{CredentialRepositoryError, CredentialRepositoryTrait},
};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;

use crate::db::dto::CredentialDto;

const TABLE_NAME: &str = "ag_oidc_credential";

pub struct CredentialRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<SqlxPostgresAdapter<L>>,
}

impl<L> CredentialRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<SqlxPostgresAdapter<L>>) -> Self {
        Self { adapter }
    }
}

#[async_trait]
impl<L> CredentialRepositoryTrait for CredentialRepository<L>
where
    L: LoggerTrait,
{
    type UserId = UniqueId;

    async fn insert(
        &self,
        credential: &Credential<Self::UserId>,
    ) -> Result<(), CredentialRepositoryError> {
        let user_id: Uuid = credential.user_id.into();

        let sql = format!(
            "INSERT INTO {TABLE_NAME} (external_id, user_id, provider, created_at) VALUES ($1, $2, $3, $4)"
        );
        let query = sqlx::query(&sql)
            .bind(credential.external_id.clone())
            .bind(user_id)
            .bind(credential.provider.clone())
            .bind(credential.created_at);

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(CredentialRepositoryError::Save(Some(Arc::new(e)))),
        }
    }

    async fn find_by_external_id(
        &self,
        external_id: &str,
        provider: &str,
    ) -> Result<Option<Credential<Self::UserId>>, CredentialRepositoryError> {
        let sql = format!(
            "SELECT external_id, user_id, provider, created_at FROM {TABLE_NAME} WHERE external_id = $1 AND provider = $2 LIMIT 1"
        );
        let query = sqlx::query_as(&sql).bind(external_id).bind(provider);

        let result: Result<Option<CredentialDto>, DbAdapterError> =
            self.adapter.fetch_one(query).await;

        match result {
            Ok(Some(dto)) => Ok(Some(Credential::<Self::UserId> {
                user_id: dto.user_id.into(),
                external_id: dto.external_id,
                provider: dto.provider,
                created_at: dto.created_at,
            })),

            Ok(None) => Ok(None),
            Err(e) => Err(CredentialRepositoryError::Find(Some(Arc::new(e)))),
        }
    }
}
