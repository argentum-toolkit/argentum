use crate::entity::credential::PasswordCredential;
use crate::repository::password_credential_repository::PasswordCredentialRepository;
use argentum_standard_business::data_type::id::Id;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct PasswordCredentialRepositoryMock {
    credentials: RwLock<HashMap<String, PasswordCredential>>,
}

impl PasswordCredentialRepositoryMock {
    pub fn new() -> Self {
        PasswordCredentialRepositoryMock {
            credentials: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for PasswordCredentialRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordCredentialRepository for PasswordCredentialRepositoryMock {
    fn save(&self, cred: &PasswordCredential) {
        self.credentials
            .write()
            .unwrap()
            .insert(cred.user_id.to_string(), cred.clone());
    }

    fn find_by_user_id(&self, id: &Id) -> Option<PasswordCredential> {
        self.credentials
            .read()
            .unwrap()
            .get(&*id.to_string())
            .map(|c| PasswordCredential::new(c.user_id.clone(), c.password.clone(), c.salt.clone()))
    }

    fn delete(&self, cred: &PasswordCredential) {
        self.credentials
            .write()
            .unwrap()
            .remove(&cred.user_id.to_string());
    }
}
