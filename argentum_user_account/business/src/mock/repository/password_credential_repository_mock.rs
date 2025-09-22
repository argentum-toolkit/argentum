use crate::entity::credential::PasswordCredential;
use crate::repository::password_credential_repository::{
    PasswordCredentialRepositoryError, PasswordCredentialRepositoryTrait,
};
use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Default)]
pub struct PasswordCredentialRepositoryMock<I>
where
    I: Id,
{
    credentials: RwLock<HashMap<String, PasswordCredential<I>>>,
}

impl<I> PasswordCredentialRepositoryMock<I>
where
    I: Id,
{
    pub fn new() -> Self {
        Self {
            credentials: RwLock::new(HashMap::new()),
        }
    }
}

#[derive(Debug)]
struct PoisonedLockError;

impl std::fmt::Display for PoisonedLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "`RwLock` is poisoned")
    }
}

impl Error for PoisonedLockError {}

#[async_trait]
impl<I> PasswordCredentialRepositoryTrait for PasswordCredentialRepositoryMock<I>
where
    I: Id,
{
    type Id = I;

    async fn save(
        &self,
        cred: &PasswordCredential<Self::Id>,
    ) -> Result<(), PasswordCredentialRepositoryError> {
        self.credentials
            .write()
            .map_err(|_| PasswordCredentialRepositoryError::Other(Arc::new(PoisonedLockError)))?
            .insert(cred.user_id.to_string(), cred.clone());

        Ok(())
    }

    async fn find_by_user_id(
        &self,
        id: Self::Id,
    ) -> Result<Option<PasswordCredential<Self::Id>>, PasswordCredentialRepositoryError> {
        let credentials = self
            .credentials
            .read()
            .map_err(|_| PasswordCredentialRepositoryError::Other(Arc::new(PoisonedLockError)))?
            .get(&*id.to_string())
            .map(|c| PasswordCredential::new(c.user_id, &c.password, &c.salt));

        Ok(credentials)
    }

    async fn delete(
        &self,
        cred: &PasswordCredential<Self::Id>,
    ) -> Result<(), PasswordCredentialRepositoryError> {
        self.credentials
            .write()
            .map_err(|_| PasswordCredentialRepositoryError::Other(Arc::new(PoisonedLockError)))?
            .remove(&cred.user_id.to_string());

        Ok(())
    }
}
