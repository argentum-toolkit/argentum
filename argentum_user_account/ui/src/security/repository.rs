use super::Security;

use std::str::FromStr;
use web_sys::Storage;

pub struct SecurityRepository {
    local_storage: Storage,
}

impl SecurityRepository {
    pub fn new(local_storage: Storage) -> Self {
        Self { local_storage }
    }

    pub fn clear(&self) {
        //TODO: use consts for keys
        let _ = self.local_storage.delete("x_auth_token"); //anonymous
        let _ = self.local_storage.delete("x_auth_user_token");
        let _ = self.local_storage.delete("x_auth_user_id");
    }

    pub fn read(&self) -> Option<Security> {
        let anonymous_token = self.local_storage.get_item("x_auth_token").unwrap_or(None);

        let user_token = self
            .local_storage
            .get_item("x_auth_user_token")
            .unwrap_or(None);
        let user_id = self
            .local_storage
            .get_item("x_auth_user_id")
            .unwrap_or(None)
            .and_then(|id| uuid::Uuid::from_str(id.as_str()).ok());

        let authenticated = match (user_token.clone(), user_id) {
            (Some(token), Some(id)) => Some(Security::Authenticated(token, id)),
            _ => None,
        };

        if let Some(a) = authenticated {
            Some(a)
        } else if let Some(token) = anonymous_token {
            Some(Security::Anonymous(token))
        } else {
            self.clear();

            None
        }
    }

    pub fn save(&self, security: Security) {
        match security {
            Security::Anonymous(hash) => {
                let _ = self.local_storage.delete("x_auth_user_token");
                let _ = self.local_storage.delete("x_auth_user_id");

                let _ = self.local_storage.set("x_auth_token", &hash);
            }
            Security::Authenticated(hash, user_id) => {
                let _ = self.local_storage.delete("x_auth_token"); //anonymous

                let _ = self.local_storage.set("x_auth_user_token", &hash);
                let _ = self
                    .local_storage
                    .set("x_auth_user_id", &user_id.to_string());
            }
        };
    }
}
