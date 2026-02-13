use crate::repository::password_credential_repository::{
    PasswordCredentialRepositoryError, PasswordCredentialRepositoryTrait,
};
use argentum_encryption_business::password::{EncryptionError, Validator};
use argentum_standard_business::data_type::id::Id;
use secrecy::SecretString;
use std::sync::Arc;

pub struct PasswordCredentialChecker<I>
where
    I: Id,
{
    repository: Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>,
    validator: Arc<dyn Validator>,
}

impl<I> PasswordCredentialChecker<I>
where
    I: Id,
{
    pub fn new(
        repository: Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>,
        validator: Arc<dyn Validator>,
    ) -> Self {
        PasswordCredentialChecker {
            repository,
            validator,
        }
    }

    pub async fn check(
        &self,
        user_id: I,
        password: &SecretString,
    ) -> Result<bool, PasswordCredentialCheckerError> {
        match self.repository.find_by_user_id(user_id).await {
            Ok(None) => Ok(false),
            Ok(Some(cred)) => {
                let res = self
                    .validator
                    .validate(password, &cred.salt, &cred.password);

                match res {
                    Ok(r) => Ok(r),
                    Err(e) => Err(PasswordCredentialCheckerError::Validation(e)),
                }
            }
            Err(e) => Err(PasswordCredentialCheckerError::Repository(e)),
        }
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum PasswordCredentialCheckerError {
    #[error("Can't check credentials")]
    Repository(#[source] PasswordCredentialRepositoryError),

    #[error("Validation error")]
    Validation(#[source] EncryptionError),
}
