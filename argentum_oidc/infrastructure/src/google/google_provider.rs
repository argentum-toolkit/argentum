use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_oidc_business::{
    data_type::ExternalUserInfo,
    service::provider::{ProviderError, ProviderStrategyTrait},
};
use argentum_standard_business::data_type::EmailAddress;
use argentum_user_business::data_type::NamePart;
use async_trait::async_trait;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum GoogleTokenResponse {
    Success(GoogleTokenSuccessResponse),
    Error(GoogleTokenErrorResponse),
}

#[derive(Debug, Deserialize)]
pub struct GoogleTokenSuccessResponse {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleTokenErrorResponse {
    error: String,
    error_description: String,
}

#[derive(Deserialize, Debug)]
struct GoogleUser {
    id: String,
    name: Option<String>,
    email: Option<String>,
}

pub struct GoogleProvider<L>
where
    L: LoggerTrait,
{
    google_client_id: String,
    google_client_secret: String,
    client: Arc<reqwest::Client>,
    logger: Arc<L>,
}

impl<L> GoogleProvider<L>
where
    L: LoggerTrait,
{
    pub fn new(
        google_client_id: &str,
        google_client_secret: &str,
        client: Arc<reqwest::Client>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            google_client_id: google_client_id.into(),
            google_client_secret: google_client_secret.into(),
            client,
            logger,
        }
    }
}

#[async_trait]
impl<L> ProviderStrategyTrait for GoogleProvider<L>
where
    L: LoggerTrait,
{
    fn get_provider_name(&self) -> &'static str {
        "Google"
    }

    async fn exchange_code(&self, code: &str) -> Result<String, ProviderError> {
        let r = "http://localhost:8080/oidc/auth/callback".to_string();
        //TODO: get url from params
        let redirect_url = r + "?provider=Google";

        let params = [
            ("client_id", self.google_client_id.as_str()),
            ("client_secret", self.google_client_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_url.as_str()),
        ];

        let token_result = self
            .client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .header("Accept", "application/json")
            .send()
            .await;

        let token_resp = match token_result {
            Ok(resp) => resp,
            Err(e) => {
                self.logger
                    .error(format!("Can't exchange code. Network error: {}", e));

                return Err(ProviderError::Network(Arc::new(e)));
            }
        };

        let status = token_resp.status();

        if !status.is_success() {
            let token_result = token_resp.text().await;
            return Err(ProviderError::BadStatus(
                status.as_u16(),
                token_result.unwrap_or_default(),
            ));
        }

        let token_result: GoogleTokenResponse = match token_resp.json().await {
            Ok(r) => r,
            Err(e) => {
                self.logger
                    .error(format!("Can't exchange code. Network error: {}", e));
                return Err(ProviderError::Network(Arc::new(e)));
            }
        };

        let token_data: GoogleTokenSuccessResponse = match token_result {
            GoogleTokenResponse::Success(success) => success,
            GoogleTokenResponse::Error(error) => {
                return Err(ProviderError::TokenData(format!(
                    "Cant exchange code: Error: {}; Error description: {}",
                    error.error, error.error_description,
                )));
            }
        };

        if token_data.token_type.to_lowercase() != "bearer" {
            return Err(ProviderError::TokenData(format!(
                "Unexpected token type: {}",
                token_data.token_type,
            )));
        }

        Ok(token_data.access_token)
    }

    async fn get_user_info(&self, token: &str) -> Result<ExternalUserInfo, ProviderError> {
        let user_result = self
            .client
            .get("https://www.googleapis.com/oauth2/v1/userinfo")
            .bearer_auth(token)
            .header("Accept", "application/json")
            .send()
            .await;

        let user_resp = match user_result {
            Ok(r) => r,
            Err(e) => {
                self.logger
                    .error(format!("Can't get user from OIDC. Network error: {}", e));

                return Err(ProviderError::Network(Arc::new(e)));
            }
        };

        let user_json_result = user_resp.json().await;

        let user: GoogleUser = match user_json_result {
            Ok(u) => u,
            Err(e) => {
                self.logger
                    .error(format!("Can't get user from OIDC. Network error: {}", e));

                return Err(ProviderError::Network(Arc::new(e)));
            }
        };

        let email = match user.email {
            Some(email) => match EmailAddress::try_new(&email) {
                Ok(email) => email,
                Err(e) => {
                    return Err(ProviderError::TokenData(format!(
                        "Can't parse email address. Error: `{0}`",
                        e
                    )));
                }
            },
            None => return Err(ProviderError::TokenData("Email address is empty".into())),
        };

        let id = user.id;
        let default_name = NamePart::try_new("UNKNOWN")
            .map_err(|_| ProviderError::TokenData("Can't extract token data".into()))?;

        let first_name = NamePart::try_new(&user.name.unwrap_or(id.clone()))
            .unwrap_or(NamePart::try_new(&id).unwrap_or(default_name));

        Ok(ExternalUserInfo::new(&id, email, first_name, None))
    }
}
