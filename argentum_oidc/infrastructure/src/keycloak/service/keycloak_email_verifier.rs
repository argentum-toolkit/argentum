use std::sync::Arc;

use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::use_case::member_confirms_email::{
    MemberConfirmsEmailError, MemberConfirmsEmailUcTrait,
};
use async_trait::async_trait;

pub struct KeycloakEmailVerifier {
    base_url: String,
    realm: String,

    client: Arc<reqwest::Client>,
}

impl KeycloakEmailVerifier {
    pub fn new(base_url: &str, realm: &str, client: Arc<reqwest::Client>) -> Self {
        Self {
            base_url: base_url.into(),
            realm: realm.into(),
            client,
        }
    }
}

#[async_trait]
impl MemberConfirmsEmailUcTrait for KeycloakEmailVerifier {
    type Id = UniqueId;
    async fn execute(&self, token: &str) -> Result<(), MemberConfirmsEmailError> {
        let url = format!(
            "{}/realms/{}/login-actions/action?key={}",
            self.base_url, self.realm, token
        );

        let resp = self.client.post(&url).send().await.unwrap();

        if resp.status().is_success() {
            Ok(())
        } else {
            match resp.text().await {
                Ok(t) => Err(MemberConfirmsEmailError::Other(t)),
                Err(e) => Err(MemberConfirmsEmailError::Network(
                    Arc::<reqwest::Error>::new(e),
                )),
            }
        }
    }
}
