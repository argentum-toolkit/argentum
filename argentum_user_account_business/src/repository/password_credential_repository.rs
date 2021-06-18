use crate::entity::credential::PasswordCredential;
use argentum_standard_business::data_type::id::Id;

pub trait PasswordCredentialRepository {
    fn save(&self, cred: &PasswordCredential);

    fn find_by_user_id(&self, id: &Id) -> Option<PasswordCredential>;

    fn delete(&self, cred: &PasswordCredential);
}
