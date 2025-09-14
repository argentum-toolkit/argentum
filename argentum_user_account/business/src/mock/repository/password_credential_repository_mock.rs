use crate::entity::credential::PasswordCredential;
use crate::repository::password_credential_repository::{
    PasswordCredentialRepositoryError, PasswordCredentialRepositoryTrait,
};
use argentum_standard_business::data_type::id::Id;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;

pub struct PasswordCredentialRepositoryMock {
    credentials: RwLock<HashMap<String, PasswordCredential>>,
}

impl PasswordCredentialRepositoryMock {
    pub fn new() -> Self {
        PasswordCredentialRepositoryMock {
            credentials: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for PasswordCredentialRepositoryMock {
    fn default() -> Self {
        Self::new()
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

impl PasswordCredentialRepositoryTrait for PasswordCredentialRepositoryMock {
    fn save(&self, cred: &PasswordCredential) -> Result<(), PasswordCredentialRepositoryError> {
        self.credentials
            .write()
            .map_err(|_| PasswordCredentialRepositoryError::Other(Arc::new(PoisonedLockError)))?
            .insert(cred.user_id.to_string(), cred.clone());

        Ok(())
    }

    fn find_by_user_id(
        &self,
        id: &Id,
    ) -> Result<Option<PasswordCredential>, PasswordCredentialRepositoryError> {
        let credentials = self
            .credentials
            .read()
            .map_err(|_| PasswordCredentialRepositoryError::Other(Arc::new(PoisonedLockError)))?
            .get(&*id.to_string())
            .map(|c| {
                PasswordCredential::new(c.user_id.clone(), c.password.clone(), c.salt.clone())
            });

        Ok(credentials)
    }

    fn delete(&self, cred: &PasswordCredential) -> Result<(), PasswordCredentialRepositoryError> {
        self.credentials
            .write()
            .map_err(|_| PasswordCredentialRepositoryError::Other(Arc::new(PoisonedLockError)))?
            .remove(&cred.user_id.to_string());

        Ok(())
    }
}
