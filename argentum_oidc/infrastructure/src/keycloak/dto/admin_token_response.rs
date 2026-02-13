use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AdminTokenResponse {
    pub(crate) access_token: String,
    pub(crate) expires_in: u32, // Seconds
}
