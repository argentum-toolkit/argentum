use crate::entity::credential::PasswordCredential;
use crate::repository::password_credential_repository::PasswordCredentialRepository;
use argentum_standard_business::data_type::id::IdTrait;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

/// TODO: NTS!!!!1111
pub struct PasswordCredentialRepositoryMock<'a> {
    credentials: RefCell<HashMap<String, PasswordCredential>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> PasswordCredentialRepositoryMock<'a> {
    pub fn new() -> Self {
        PasswordCredentialRepositoryMock {
            credentials: RefCell::new(HashMap::new()),
            phantom: Default::default(),
        }
    }
}

impl<'a> Default for PasswordCredentialRepositoryMock<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> PasswordCredentialRepository<'a> for PasswordCredentialRepositoryMock<'a> {
    fn save(&self, cred: &PasswordCredential) {
        self.credentials
            .borrow_mut()
            .insert(cred.user_id.to_string(), cred.clone());
    }

    fn find_by_user_id(&self, id: &Box<dyn IdTrait>) -> Option<PasswordCredential> {
        self.credentials
            .borrow()
            .get(&*id.to_string())
            .map(|c| PasswordCredential::new(c.user_id.clone(), c.password.clone(), c.salt.clone()))
    }
}
