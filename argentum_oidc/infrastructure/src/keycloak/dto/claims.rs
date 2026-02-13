use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) sub: String, // user_id
    pub(crate) preferred_username: Option<String>,
    pub(crate) email: Option<String>, //we use email as username
}
