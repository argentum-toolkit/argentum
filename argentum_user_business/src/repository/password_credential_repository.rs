use crate::entity::credential::PasswordCredential;
use argentum_standard_business::data_type::id::IdTrait;

pub trait PasswordCredentialRepository<'a> {
    fn save(&self, cred: &PasswordCredential);

    fn find_by_user_id(&self, id: &Box<dyn IdTrait>) -> Option<PasswordCredential>;
}
