use std::sync::Arc;

use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::{
    data_type::builder::NameBuilder,
    entity::user::Member,
    service::{MemberAuthenticatorError, MemberAuthenticatorTrait},
};
use serde::Deserialize;
use serde_json::Value;

use async_trait::async_trait;
use uuid::Uuid;

#[derive(Deserialize)]
struct MemberDto {
    #[serde(rename = "sub")]
    pub id: Uuid,

    // #[serde(rename = "name")]
    // pub created_at: DateTime<Utc>,
    #[serde(rename = "given_name")]
    pub first_name: String,

    #[serde(rename = "family_name")]
    pub last_name: Option<String>,

    pub email: String,
}

pub struct KeycloakMemberAuthenticator {
    base_url: String,
    realm: String,
    client_id: String,
    client_secret: String,
    client: Arc<reqwest::Client>,
}

impl KeycloakMemberAuthenticator {
    pub(crate) fn new(
        base_url: &str,
        realm: &str,
        client_id: &str,
        client_secret: &str,
        client: Arc<reqwest::Client>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            realm: realm.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            client,
        }
    }
}

#[async_trait]
impl MemberAuthenticatorTrait for KeycloakMemberAuthenticator {
    type Id = UniqueId;

    async fn authenticate(
        &self,
        token: &str,
    ) -> Result<Option<Member<Self::Id>>, MemberAuthenticatorError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/token/introspect",
            self.base_url, self.realm
        );

        //TODO: unwrap
        let res = self
            .client
            .post(url)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&[("token", token)])
            .send()
            .await
            .unwrap();

        let json: Value = res.json().await.unwrap();

        match serde_json::from_value::<MemberDto>(json.clone()) {
            Ok(m) => {
                let email = EmailAddress::try_new(&m.email).unwrap();
                let name = NameBuilder::new(&m.first_name)
                    .last(m.last_name.as_deref())
                    .try_build()
                    .unwrap();

                Ok(Some(Member::new(m.id.into(), name, email, true)))
            }
            Err(_) => Ok(None),
        }
    }
}
