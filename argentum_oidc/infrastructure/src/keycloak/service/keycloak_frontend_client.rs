use std::sync::Arc;

use argentum_oidc_business::{
    data_type::ExchangedTokenData,
    service::{AuthCodeExchangerError, AuthCodeExchangerTrait},
};
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use async_trait::async_trait;
use jsonwebtoken::{TokenData, dangerous::insecure_decode};
use uuid::Uuid;

use std::str::FromStr;

use crate::keycloak::dto::{Claims, TokenResponse};

pub struct KeycloakFrontendClient {
    base_url: String,
    realm: String,
    client_id: String,
    redirect_uri: String,

    client: Arc<reqwest::Client>,
}

impl KeycloakFrontendClient {
    pub fn new(
        base_url: &str,
        realm: &str,
        client_id: &str,
        redirect_uri: &str, // Frontend callback
        client: Arc<reqwest::Client>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            realm: realm.into(),
            client_id: client_id.into(),
            redirect_uri: redirect_uri.into(),
            client,
        }
    }
}

//TODO:  log errors

#[async_trait]
impl AuthCodeExchangerTrait for KeycloakFrontendClient {
    type UserId = UniqueId;

    async fn exchange_code(
        &self,
        code: &str,
    ) -> Result<ExchangedTokenData<Self::UserId>, AuthCodeExchangerError> {
        let url = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.base_url, self.realm
        );

        let params = [
            ("grant_type", "authorization_code"),
            ("client_id", &self.client_id),
            ("redirect_uri", &self.redirect_uri), // Frontend callback
            ("code", code),
        ];

        match self.client.post(&url).form(&params).send().await {
            Ok(response) if response.status().is_success() => {
                match response.json::<TokenResponse>().await {
                    Ok(token_info) => {
                        let access_token = match token_info.access_token {
                            Some(token) => token,
                            None => {
                                return Err(AuthCodeExchangerError::TokenData(
                                    "Can't create an access token".into(),
                                ));
                            }
                        };

                        let token_data: TokenData<Claims> = match insecure_decode(&access_token) {
                            Ok(data) => data,
                            Err(e) => {
                                return Err(AuthCodeExchangerError::TokenData(format!(
                                    "Can't parse access token. Error: `{0}`",
                                    e
                                )));
                            }
                        };

                        let user_uuid = match Uuid::from_str(&token_data.claims.sub) {
                            Ok(id) => id,
                            Err(e) => {
                                return Err(AuthCodeExchangerError::TokenData(format!(
                                    "Can't parse UUID. Error: `{0}`",
                                    e
                                )));
                            }
                        };

                        let user_id = UniqueId::from(user_uuid);

                        //be sure that email is valid
                        let _email = match token_data.claims.email {
                            Some(raw_email) => match EmailAddress::try_new(&raw_email) {
                                Ok(email) => email,
                                Err(e) => {
                                    return Err(AuthCodeExchangerError::TokenData(format!(
                                        "Can't parse email address. Error: `{0}`",
                                        e
                                    )));
                                }
                            },
                            None => {
                                return Err(AuthCodeExchangerError::TokenData(
                                    "Email address is empty".into(),
                                ));
                            }
                        };

                        Ok(ExchangedTokenData::new(&access_token, user_id))
                    }

                    Err(_) => Err(AuthCodeExchangerError::Network),
                }
            }
            Ok(response) => Err(AuthCodeExchangerError::OidcBadStatus(
                response.status().as_u16(),
            )),
            Err(_) => Err(AuthCodeExchangerError::Network),
        }
    }
}
