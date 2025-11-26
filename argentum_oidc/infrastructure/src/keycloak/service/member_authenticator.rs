use std::sync::Arc;

use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::{
    data_type::builder::NameBuilder,
    entity::user::AuthenticatedUser,
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
    client: Arc<reqwest::Client>,
}

impl KeycloakMemberAuthenticator {
    pub(crate) fn new(client: Arc<reqwest::Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl MemberAuthenticatorTrait for KeycloakMemberAuthenticator {
    type Id = UniqueId;

    async fn authenticate(
        &self,
        token: &str,
    ) -> Result<Option<AuthenticatedUser<Self::Id>>, MemberAuthenticatorError> {
        //TODO: move string data into config
        let res = self.client
            .post("http://service.keycloak:8080/realms/argentum-realm-dev/protocol/openid-connect/token/introspect")
            .basic_auth("argentum-be-client-dev", Some("dj9cobGEQptoEk5gMvqadYHd6emeUQ2Y"))
            .form(&[("token", token)])
            .send()
            .await
            .unwrap()
        ;

        let json: Value = res.json().await.unwrap();

        match serde_json::from_value::<MemberDto>(json.clone()) {
            Ok(m) => {
                let email = EmailAddress::try_new(&m.email).unwrap();
                let name = NameBuilder::new(&m.first_name)
                    .last(m.last_name.as_deref())
                    .try_build()
                    .unwrap();

                Ok(Some(AuthenticatedUser::new(m.id.into(), name, email)))
            }
            Err(_) => Ok(None),
        }
    }
}
