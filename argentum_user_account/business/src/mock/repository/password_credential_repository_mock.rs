use crate::entity::credential::{EncodedPasswordCredential, PasswordCredential};
use crate::repository::password_credential_repository::{
    PasswordCredentialRepositoryError, PasswordCredentialRepositoryTrait,
};
use argentum_encryption_business::password::Encryptor;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::error::PoisonedLockError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub struct PasswordCredentialRepositoryMock<I>
where
    I: Id,
{
    credentials: RwLock<HashMap<String, EncodedPasswordCredential<I>>>,

    encryptor: Arc<dyn Encryptor>,
}

impl<I> PasswordCredentialRepositoryMock<I>
where
    I: Id,
{
    pub fn new(encryptor: Arc<dyn Encryptor>) -> Self {
        Self {
            credentials: RwLock::new(HashMap::new()),
            encryptor,
        }
    }
}

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
        let (password, salt) = self.encryptor.encrypt(&cred.password)?;
        let encoded = EncodedPasswordCredential::new(cred.user_id, password.clone(), salt);

        self.credentials
            .write()
            .map_err(|_| PasswordCredentialRepositoryError::Other(Arc::new(PoisonedLockError)))?
            .insert(encoded.user_id.to_string(), encoded);

        Ok(())
    }

    async fn find_by_user_id(
        &self,
        id: Self::Id,
    ) -> Result<Option<EncodedPasswordCredential<Self::Id>>, PasswordCredentialRepositoryError>
    {
        let credentials = self
            .credentials
            .read()
            .map_err(|_| PasswordCredentialRepositoryError::Other(Arc::new(PoisonedLockError)))?
            .get(&*id.to_string())
            .map(|c| EncodedPasswordCredential::new(c.user_id, c.password.clone(), c.salt.clone()));

        Ok(credentials)
    }

    async fn delete(
        &self,
        cred: EncodedPasswordCredential<Self::Id>,
    ) -> Result<(), PasswordCredentialRepositoryError> {
        self.credentials
            .write()
            .map_err(|_| PasswordCredentialRepositoryError::Other(Arc::new(PoisonedLockError)))?
            .remove(&cred.user_id.to_string());

        Ok(())
    }
}
