use crate::entity::credential::Credential;

pub trait CredentialWriterTrait {
    fn write(&self, cred: Box<dyn Credential>);
}
