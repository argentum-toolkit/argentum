use argentum_standard_business::data_type::id::Id;
use std::any::Any;

pub trait Credential {
    fn as_any(&self) -> &dyn Any;
}

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

impl Clone for PasswordCredential {
    fn clone(&self) -> PasswordCredential {
        PasswordCredential {
            user_id: self.user_id.clone(),
            password: self.password.clone(),
            salt: self.salt.clone(),
        }
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
