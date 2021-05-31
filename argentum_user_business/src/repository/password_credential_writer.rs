use crate::entity::credential::{Credential, PasswordCredential};
use crate::repository::credential_writer::CredentialWriterTrait;
use crate::repository::password_credential_repository::PasswordCredentialRepository;

pub trait PasswordCredentialWriterTrait<'s>: CredentialWriterTrait {
    fn write_password_credentials(&self, cred: &PasswordCredential);
}

impl<'s, T: PasswordCredentialWriterTrait<'s>> CredentialWriterTrait for T {
    fn write(&self, cred: Box<dyn Credential>) {
        let pass_cred = match cred.as_any().downcast_ref::<PasswordCredential>() {
            Some(b) => b,
            None => panic!("Accepted only PasswordCredential type"),
        };

        self.write_password_credentials(pass_cred);
    }
}

pub struct PasswordCredentialWriter<'s> {
    repository: &'s dyn PasswordCredentialRepository,
}

impl<'s> PasswordCredentialWriter<'s> {
    pub fn new(repository: &'s dyn PasswordCredentialRepository) -> Self {
        PasswordCredentialWriter { repository }
    }
}

impl<'s> PasswordCredentialWriterTrait<'s> for PasswordCredentialWriter<'s> {
    fn write_password_credentials(&self, cred: &PasswordCredential) {
        self.repository.save(cred);
    }
}
