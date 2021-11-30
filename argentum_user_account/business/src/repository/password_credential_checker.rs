use crate::repository::password_credential_repository::{
    PasswordCredentialRepositoryError, PasswordCredentialRepositoryTrait,
};
use argentum_encryption_business::password::Validator;
use argentum_standard_business::data_type::id::Id;
use std::sync::Arc;

pub struct PasswordCredentialChecker {
    repository: Arc<dyn PasswordCredentialRepositoryTrait>,
    validator: Arc<dyn Validator>,
}

impl PasswordCredentialChecker {
    pub fn new(
        repository: Arc<dyn PasswordCredentialRepositoryTrait>,
        validator: Arc<dyn Validator>,
    ) -> Self {
        PasswordCredentialChecker {
            repository,
            validator,
        }
    }

    pub fn check(
        &self,
        user_id: Id,
        password: &str,
    ) -> Result<bool, PasswordCredentialCheckerError> {
        match self.repository.find_by_user_id(&user_id) {
            Ok(None) => Ok(false),
            Ok(Some(cred)) => {
                let res =
                    self.validator
                        .validate(password, cred.salt.as_str(), cred.password.as_str());

                Ok(res)
            }
            Err(e) => Err(PasswordCredentialCheckerError::Repository(e)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PasswordCredentialCheckerError {
    #[error("Can't check credentials")]
    Repository(#[source] PasswordCredentialRepositoryError),
}
