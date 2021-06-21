use crate::entity::credential::Credential;
use argentum_standard_business::data_type::id::Id;

pub trait CredentialWriterTrait {
    fn write(&self, cred: Box<dyn Credential>);

    fn delete_for_user(&self, user_id: &Id);
}
