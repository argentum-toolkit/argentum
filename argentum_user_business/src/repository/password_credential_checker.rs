use crate::repository::password_credential_repository::PasswordCredentialRepository;
use argentum_encryption_business::password::Validator;
use argentum_standard_business::data_type::id::Id;

pub struct PasswordCredentialChecker<'s> {
    repository: &'s dyn PasswordCredentialRepository,
    validator: &'s dyn Validator,
}

impl<'s> PasswordCredentialChecker<'s> {
    pub fn new(
        repository: &'s dyn PasswordCredentialRepository,
        validator: &'s dyn Validator,
    ) -> Self {
        PasswordCredentialChecker {
            repository,
            validator,
        }
    }

    pub fn check(&self, user_id: Id, password: &str) -> bool {
        match self.repository.find_by_user_id(&user_id) {
            None => false,
            Some(cred) => {
                self.validator
                    .validate(password, &cred.salt.as_str(), &cred.password.as_str())
            }
        }
    }
}
