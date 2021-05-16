use crate::repository::password_credential_repository::PasswordCredentialRepository;
use argentum_encryption_business::password::Validator;
use argentum_standard_business::data_type::id::IdTrait;

pub struct PasswordCredentialChecker<'a> {
    repository: &'a dyn PasswordCredentialRepository<'a>,
    validator: &'a dyn Validator,
}

impl<'a> PasswordCredentialChecker<'a> {
    pub fn new(
        repository: &'a dyn PasswordCredentialRepository<'a>,
        validator: &'a dyn Validator,
    ) -> Self {
        PasswordCredentialChecker {
            repository,
            validator,
        }
    }

    pub fn check(&self, user_id: Box<dyn IdTrait>, password: &str) -> bool {
        match self.repository.find_by_user_id(&user_id) {
            None => false,
            Some(cred) => {
                self.validator
                    .validate(password, &cred.salt.as_str(), &cred.password.as_str())
            }
        }
    }
}
