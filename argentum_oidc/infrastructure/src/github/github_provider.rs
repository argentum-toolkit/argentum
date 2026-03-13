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
pub enum GithubTokenResponse {
    Success(GithubTokenSuccessResponse),
    Error(GithubTokenErrorResponse),
}

#[derive(Debug, Deserialize)]
pub struct GithubTokenSuccessResponse {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubTokenErrorResponse {
    error: String,
    error_description: String,
}

#[derive(Deserialize, Debug)]
struct GithubUser {
    id: u64,
    // login: String,
    name: Option<String>,
    email: Option<String>,
    // avatar_url: Option<String>,
}

#[derive(Deserialize, Clone)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

pub struct GitHubProvider<L>
where
    L: LoggerTrait,
{
    github_client_id: String,
    github_client_secret: String,
    client: Arc<reqwest::Client>,
    logger: Arc<L>,
}

impl<L> GitHubProvider<L>
where
    L: LoggerTrait,
{
    pub fn new(
        github_client_id: &str,
        github_client_secret: &str,
        client: Arc<reqwest::Client>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            github_client_id: github_client_id.into(),
            github_client_secret: github_client_secret.into(),
            client,
            logger,
        }
    }

    async fn get_email(
        &self,
        access_token: &str,
        user: &GithubUser,
    ) -> Result<EmailAddress, ProviderError> {
        // get list of emails (/user/emails)
        // email field in /user can be null
        let emails_result = self
            .client
            .get("https://api.github.com/user/emails")
            .bearer_auth(access_token)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await;

        let email_resp = match emails_result {
            Ok(r) => r,
            Err(e) => {
                self.logger
                    .error(format!("Can't get emails from OIDC. Network error: {}", e));

                return Err(ProviderError::Network(Arc::new(e)));
            }
        };

        let emails_result = email_resp.json().await;

        let emails: Vec<GithubEmail> = match emails_result {
            Ok(emails) => emails,
            Err(e) => {
                self.logger
                    .error(format!("Can't get email from OIDC. Network error: {}", e));

                return Err(ProviderError::Network(Arc::new(e)));
            }
        };

        // Select the best email
        let primary_email = emails
            .clone()
            .into_iter()
            .find(|e| e.primary && e.verified)
            .map(|e| e.email)
            .or_else(|| emails.into_iter().find(|e| e.verified).map(|e| e.email))
            .or_else(|| user.email.clone()); // fallback to /user if nothing better found

        //be sure that email is valid
        match primary_email {
            Some(raw_email) => match EmailAddress::try_new(&raw_email) {
                Ok(email) => Ok(email),
                Err(e) => Err(ProviderError::TokenData(format!(
                    "Can't parse email address. Error: `{0}`",
                    e
                ))),
            },
            None => Err(ProviderError::TokenData("Email address is empty".into())),
        }
    }
}

#[async_trait]
impl<L> ProviderStrategyTrait for GitHubProvider<L>
where
    L: LoggerTrait,
{
    fn get_provider_name(&self) -> &'static str {
        "GitHub"
    }

    async fn exchange_code(&self, code: &str) -> Result<String, ProviderError> {
        //TODO: check status codes. There should be not 500 for expired GitHubToken
        let params = [
            ("client_id", self.github_client_id.as_str()),
            ("client_secret", self.github_client_secret.as_str()),
            ("code", code),
            // ("code_verifier", code_verifier),
            //TODO: implement code_verifier/challenge
        ];

        let token_result = self
            .client
            .post("https://github.com/login/oauth/access_token")
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

        let token_result: GithubTokenResponse = match token_resp.json().await {
            Ok(r) => r,
            Err(e) => {
                self.logger
                    .error(format!("Can't exchange code. Network error: {}", e));
                return Err(ProviderError::Network(Arc::new(e)));
            }
        };

        let token_data: GithubTokenSuccessResponse = match token_result {
            GithubTokenResponse::Success(success) => success,
            GithubTokenResponse::Error(error) => {
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
        // Get general user information (/user)
        let user_result = self
            .client
            .get("https://api.github.com/user")
            .bearer_auth(token)
            .header("Accept", "application/vnd.github.v3+json")
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

        let user: GithubUser = match user_json_result {
            Ok(u) => u,
            Err(e) => {
                self.logger
                    .error(format!("Can't get user from OIDC. Network error: {}", e));

                return Err(ProviderError::Network(Arc::new(e)));
            }
        };

        let email = self.get_email(token, &user).await?;

        let id = user.id.to_string();
        let default_name = NamePart::try_new("UNKNOWN")
            .map_err(|_| ProviderError::TokenData("Can't extract token data".into()))?;

        let first_name = NamePart::try_new(&user.name.unwrap_or(id.clone()))
            .unwrap_or(NamePart::try_new(&id).unwrap_or(default_name));

        Ok(ExternalUserInfo::new(&id, email, first_name, None))
    }
}
