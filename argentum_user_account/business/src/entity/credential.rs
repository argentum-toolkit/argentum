use argentum_standard_business::data_type::id::Id;
use std::any::Any;

pub trait Credential {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
pub struct PasswordCredential {
    pub user_id: Id,
    pub password: String,
    pub salt: String,
}

impl Credential for PasswordCredential {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PasswordCredential {
    pub fn new(user_id: Id, password: String, salt: String) -> Self {
        PasswordCredential {
            user_id,
            password,
            salt,
        }
    }
}
