use crate::entity::credential::{Credential, PasswordCredential};
use crate::repository::credential_writer::CredentialWriterTrait;
use crate::repository::password_credential_repository::PasswordCredentialRepository;

pub trait PasswordCredentialWriterTrait<'a>: CredentialWriterTrait {
    fn write_password_credentials(&self, cred: &PasswordCredential);
}

impl<'a, T: PasswordCredentialWriterTrait<'a>> CredentialWriterTrait for T {
    fn write(&self, cred: Box<dyn Credential>) {
        let pass_cred = match cred.as_any().downcast_ref::<PasswordCredential>() {
            Some(b) => b,
            None => panic!("Accepted only PasswordCredential type"),
        };

        self.write_password_credentials(pass_cred);
    }
}

pub struct PasswordCredentialWriter<'a> {
    repository: &'a dyn PasswordCredentialRepository<'a>,
}

impl<'a> PasswordCredentialWriter<'a> {
    pub fn new(repository: &'a dyn PasswordCredentialRepository<'a>) -> Self {
        PasswordCredentialWriter { repository }
    }
}

impl<'a> PasswordCredentialWriterTrait<'a> for PasswordCredentialWriter<'a> {
    fn write_password_credentials(&self, cred: &PasswordCredential) {
        self.repository.save(cred);
    }
}
