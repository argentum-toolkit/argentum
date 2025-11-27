use std::sync::Arc;

use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::service::{
    PasswordToTokenExchangerError, PasswordToTokenExchangerTrait,
};
use argentum_user_business::entity::session::Session;
use async_trait::async_trait;
use jsonwebtoken::{TokenData, dangerous::insecure_decode};
use uuid::Uuid;

use std::str::FromStr;

use crate::keycloak::dto::{Claims, TokenResponse};
use secrecy::{ExposeSecret, SecretString};

pub struct KeycloakBackendClient {
    base_url: String,
    realm: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,

    client: Arc<reqwest::Client>,
}

impl KeycloakBackendClient {
    pub fn new(
        base_url: &str,
        realm: &str,
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str, // Frontend callback
        client: Arc<reqwest::Client>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            realm: realm.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
            client,
        }
    }
}

#[async_trait]
impl PasswordToTokenExchangerTrait for KeycloakBackendClient {
    type Id = UniqueId;

    async fn exchange_password(
        &self,
        email: &EmailAddress,
        password: &SecretString,
    ) -> Result<Session<Self::Id>, PasswordToTokenExchangerError> {
        let params = [
            ("grant_type", "password"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("redirect_uri", &self.redirect_uri), // Frontend callback
            ("username", &email.as_string()),
            ("password", password.expose_secret()),
            ("scope", "openid"),
        ];

        let response = self
            .client
            .post(format!(
                "{}/realms/{}/protocol/openid-connect/token",
                self.base_url, self.realm
            ))
            .form(&params)
            .send()
            .await;

        match response {
            Ok(response) if response.status().is_success() => {
                match response.json::<TokenResponse>().await {
                    Ok(token_info) => {
                        let access_token = match token_info.access_token {
                            Some(token) => token,
                            None => {
                                return Err(PasswordToTokenExchangerError::OtherString(
                                    "Can't create an access token".into(),
                                ));
                            }
                        };

                        let token_data: TokenData<Claims> = match insecure_decode(&access_token) {
                            Ok(data) => data,
                            Err(e) => {
                                return Err(PasswordToTokenExchangerError::OtherString(format!(
                                    "Can't parse access token. Error: `{0}`",
                                    e
                                )));
                            }
                        };

                        let user_uuid = match Uuid::from_str(&token_data.claims.sub) {
                            Ok(id) => id,
                            Err(e) => {
                                return Err(PasswordToTokenExchangerError::OtherString(format!(
                                    "Can't parse UUID. Error: `{0}`",
                                    e
                                )));
                            }
                        };

                        let user_id = UniqueId::from(user_uuid);

                        Ok(Session::new(UniqueId::new(), user_id, &access_token))
                    }

                    Err(e) => Err(PasswordToTokenExchangerError::Other(Some(Arc::new(e)))),
                }
            }
            Ok(response) => Err(PasswordToTokenExchangerError::OtherString(format!(
                "Wrong status code of OIDC service. Status code: `{0}`",
                response.status().as_u16()
            ))),
            Err(e) => Err(PasswordToTokenExchangerError::Other(Some(Arc::new(e)))),
        }
    }
}
