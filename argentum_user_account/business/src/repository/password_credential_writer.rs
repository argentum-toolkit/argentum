use crate::entity::credential::{Credential, PasswordCredential};
use crate::repository::credential_writer::{
    CredentialWriterError, CredentialWriterResult, CredentialWriterTrait,
};
use crate::repository::password_credential_repository::{
    PasswordCredentialRepositoryError, PasswordCredentialRepositoryTrait,
};
use argentum_standard_business::data_type::id::Id;
use std::sync::Arc;

pub trait PasswordCredentialWriterTrait: CredentialWriterTrait + Send + Sync {
    fn write_password_credentials(
        &self,
        cred: &PasswordCredential,
    ) -> Result<(), PasswordCredentialWriterError>;

    fn delete_password_credentials_for_user(
        &self,
        user_id: &Id,
    ) -> Result<(), PasswordCredentialWriterError>;
}

impl<T: PasswordCredentialWriterTrait> CredentialWriterTrait for T {
    fn write(&self, cred: Box<dyn Credential>) -> CredentialWriterResult {
        let pass_cred = match cred.as_any().downcast_ref::<PasswordCredential>() {
            Some(b) => b,
            None => panic!("Accepted only PasswordCredential type"),
        };

        self.write_password_credentials(pass_cred)
            .map_err(|e| CredentialWriterError::Credential(Box::new(e)))
    }

    fn delete_for_user(&self, user_id: &Id) -> CredentialWriterResult {
        self.delete_password_credentials_for_user(user_id)
            .map_err(|e| CredentialWriterError::Credential(Box::new(e)))
    }
}

pub struct PasswordCredentialWriter {
    repository: Arc<dyn PasswordCredentialRepositoryTrait>,
}

impl PasswordCredentialWriter {
    pub fn new(repository: Arc<dyn PasswordCredentialRepositoryTrait>) -> Self {
        PasswordCredentialWriter { repository }
    }
}

impl PasswordCredentialWriterTrait for PasswordCredentialWriter {
    fn write_password_credentials(
        &self,
        cred: &PasswordCredential,
    ) -> Result<(), PasswordCredentialWriterError> {
        if let Err(e) = self.repository.save(cred) {
            return Err(PasswordCredentialWriterError::Repository(e));
        }

        Ok(())
    }

    fn delete_password_credentials_for_user(
        &self,
        user_id: &Id,
    ) -> Result<(), PasswordCredentialWriterError> {
        if let Some(cred) = self.repository.find_by_user_id(user_id)? {
            self.repository.delete(&cred)?;
        }

        match self.repository.find_by_user_id(user_id)? {
            Some(cred) => {
                self.repository.delete(&cred)?;
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
