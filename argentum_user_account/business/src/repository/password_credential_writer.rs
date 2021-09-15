use crate::entity::credential::{Credential, PasswordCredential};
use crate::repository::credential_writer::CredentialWriterTrait;
use crate::repository::password_credential_repository::PasswordCredentialRepository;
use argentum_standard_business::data_type::id::Id;
use std::sync::Arc;

pub trait PasswordCredentialWriterTrait: CredentialWriterTrait + Send + Sync {
    fn write_password_credentials(&self, cred: &PasswordCredential);
    fn delete_password_credentials_for_user(&self, user_id: &Id);
}

impl<T: PasswordCredentialWriterTrait> CredentialWriterTrait for T {
    fn write(&self, cred: Box<dyn Credential>) {
        let pass_cred = match cred.as_any().downcast_ref::<PasswordCredential>() {
            Some(b) => b,
            None => panic!("Accepted only PasswordCredential type"),
        };

        self.write_password_credentials(pass_cred);
    }

    fn delete_for_user(&self, user_id: &Id) {
        self.delete_password_credentials_for_user(user_id)
    }
}

pub struct PasswordCredentialWriter {
    repository: Arc<dyn PasswordCredentialRepository>,
}

impl PasswordCredentialWriter {
    pub fn new(repository: Arc<dyn PasswordCredentialRepository>) -> Self {
        PasswordCredentialWriter { repository }
    }
}

impl PasswordCredentialWriterTrait for PasswordCredentialWriter {
    fn write_password_credentials(&self, cred: &PasswordCredential) {
        self.repository.save(cred);
    }

    fn delete_password_credentials_for_user(&self, user_id: &Id) {
        if let Some(cred) = self.repository.find_by_user_id(user_id) {
            self.repository.delete(&cred);
        }
    }
}
