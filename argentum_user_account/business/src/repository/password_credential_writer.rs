use crate::entity::credential::PasswordCredential;
use crate::repository::credential_writer::{CredentialWriterError, CredentialWriterResult};
use crate::repository::password_credential_repository::{
    PasswordCredentialRepositoryError, PasswordCredentialRepositoryTrait,
};
use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait PasswordCredentialWriterTrait: Send + Sync {
    type Id: Id;

    async fn write_password_credentials(
        &self,
        cred: &PasswordCredential<<Self as PasswordCredentialWriterTrait>::Id>,
    ) -> Result<(), PasswordCredentialWriterError>;

    async fn delete_password_credentials_for_user(
        &self,
        user_id: <Self as PasswordCredentialWriterTrait>::Id,
    ) -> Result<(), PasswordCredentialWriterError>;

    async fn write(&self, cred: &PasswordCredential<Self::Id>) -> CredentialWriterResult {
        self.write_password_credentials(cred)
            .await
            .map_err(|e| CredentialWriterError::Credential(Arc::new(e)))
    }

    async fn delete_for_user(&self, user_id: Self::Id) -> CredentialWriterResult {
        self.delete_password_credentials_for_user(user_id)
            .await
            .map_err(|e| CredentialWriterError::Credential(Arc::new(e)))
    }
}

pub struct PasswordCredentialWriter<I>
where
    I: Id,
{
    repository: Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>,
}

impl<I> PasswordCredentialWriter<I>
where
    I: Id,
{
    pub fn new(repository: Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>) -> Self {
        PasswordCredentialWriter { repository }
    }
}

#[async_trait]
impl<I> PasswordCredentialWriterTrait for PasswordCredentialWriter<I>
where
    I: Id,
{
    type Id = I;

    async fn write_password_credentials(
        &self,
        cred: &PasswordCredential<<Self as PasswordCredentialWriterTrait>::Id>,
    ) -> Result<(), PasswordCredentialWriterError> {
        if let Err(e) = self.repository.save(cred).await {
            return Err(PasswordCredentialWriterError::Repository(e));
        }

        Ok(())
    }

    async fn delete_password_credentials_for_user(
        &self,
        user_id: <Self as PasswordCredentialWriterTrait>::Id,
    ) -> Result<(), PasswordCredentialWriterError> {
        if let Some(cred) = self.repository.find_by_user_id(user_id).await? {
            self.repository.delete(&cred).await?;
        }

        match self.repository.find_by_user_id(user_id).await? {
            Some(cred) => {
                self.repository.delete(&cred).await?;
                Ok(())
            }
            None => Ok(()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PasswordCredentialWriterError {
    #[error("Can't write credentials")]
    Repository(
        #[source]
        #[from]
        PasswordCredentialRepositoryError,
    ),
}
