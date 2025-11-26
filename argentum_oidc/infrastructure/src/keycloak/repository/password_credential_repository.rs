use std::error::Error;
use std::fmt::{self, Display};
use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::entity::credential::PasswordCredential;
use argentum_user_account_business::repository::password_credential_repository::{
    PasswordCredentialRepositoryError, PasswordCredentialRepositoryTrait,
};
use async_trait::async_trait;
use serde_json::json;

use crate::keycloak::service::AdminApi;

#[derive(Debug)]
struct KeycloakCredentialError(String);
impl Error for KeycloakCredentialError {}

impl Display for KeycloakCredentialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct PasswordCredentialRepository<L>
where
    L: LoggerTrait,
{
    admin_api: Arc<AdminApi<L>>,
    logger: Arc<L>,
}

impl<L> PasswordCredentialRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(admin_api: Arc<AdminApi<L>>, logger: Arc<L>) -> Self {
        Self { admin_api, logger }
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
        let path = format!("/users/{}/reset-password", cred.user_id);

        let password_body = json!({
            "type": "password",
            "value": cred.password,
            "temporary": false
        });

        let resp = self
            .admin_api
            .put(&path, Some(&password_body))
            .await
            .map_err(|e| PasswordCredentialRepositoryError::Save(Arc::new(e)))?;

        if resp.status().is_success() {
            Ok(())
        } else {
            self.logger.error(format!(
                "Failed to create user: {}",
                resp.text()
                    .await
                    .unwrap_or("Also can't get response text.".into())
            ));

            Err(PasswordCredentialRepositoryError::Save(Arc::new(
                KeycloakCredentialError(
                    "Failed to create credentials. Wrong keycloak response status code".into(),
                ),
            )))
        }
    }

    async fn find_by_user_id(
        &self,
        user_id: Self::Id,
    ) -> Result<Option<PasswordCredential<Self::Id>>, PasswordCredentialRepositoryError> {
        //TODO: implement or refactor
        todo!()
    }

    async fn delete(
        &self,
        cred: &PasswordCredential<Self::Id>,
    ) -> Result<(), PasswordCredentialRepositoryError> {
        //TODO: implement or refactor
        todo!()
    }
}
