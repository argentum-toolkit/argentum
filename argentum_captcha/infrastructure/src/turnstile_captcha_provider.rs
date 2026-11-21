use std::{sync::Arc, time::Duration};

use argentum_captcha_business::{CaptchaError, data_type::Captcha, service::CaptchaProviderTrait};
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

const TURNSTILE_SITEVERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
const TURNSTILE_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Deserialize)]
struct TurnstileSiteverifyResponse {
    success: bool,
}

pub struct TurnstileCaptchaProvider {
    secret_key: SecretString,
    client: Arc<reqwest::Client>,
}

impl TurnstileCaptchaProvider {
    pub fn new(secret_key: SecretString, client: Arc<reqwest::Client>) -> Self {
        Self { secret_key, client }
    }
}

#[async_trait]
impl CaptchaProviderTrait for TurnstileCaptchaProvider {
    async fn verify(&self, captcha: &Captcha) -> Result<bool, CaptchaError> {
        let params = [
            ("secret", self.secret_key.expose_secret()),
            ("response", captcha.as_str()),
        ];

        let response = self
            .client
            .post(TURNSTILE_SITEVERIFY_URL)
            .form(&params)
            .timeout(TURNSTILE_REQUEST_TIMEOUT)
            .send()
            .await
            .map_err(|e| unavailable(format!("network error: {e}")))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| unavailable(format!("can't read response: {e}")))?;

        check_result_from_response(status.as_u16(), &body)
    }
}

fn check_result_from_response(status: u16, body: &str) -> Result<bool, CaptchaError> {
    if !(200..300).contains(&status) {
        return Err(unavailable(format!("unexpected HTTP status {status}")));
    }

    let parsed: TurnstileSiteverifyResponse = serde_json::from_str(body)
        .map_err(|e| unavailable(format!("unexpected response body: {e}")))?;

    Ok(parsed.success)
}

fn unavailable(reason: String) -> CaptchaError {
    CaptchaError::ProviderUnavailable(format!("Turnstile: {reason}"))
}

#[cfg(test)]
mod tests {
    use super::check_result_from_response;
    use argentum_captcha_business::CaptchaError;

    #[test]
    fn successful_response_passes() {
        let result = check_result_from_response(
            200,
            r#"{"success": true, "hostname": "example.com", "error-codes": []}"#,
        );

        assert_eq!(result.ok(), Some(true));
    }

    #[test]
    fn unsuccessful_response_fails() {
        let result = check_result_from_response(
            200,
            r#"{"success": false, "error-codes": ["invalid-input-response"]}"#,
        );

        assert_eq!(result.ok(), Some(false));
    }

    #[test]
    fn invalid_json_is_provider_unavailable_not_passed() {
        let result = check_result_from_response(200, "<html>Service unavailable</html>");

        assert!(matches!(result, Err(CaptchaError::ProviderUnavailable(_))));
    }

    #[test]
    fn json_without_success_flag_is_provider_unavailable_not_passed() {
        let result = check_result_from_response(200, r#"{"hostname": "example.com"}"#);

        assert!(matches!(result, Err(CaptchaError::ProviderUnavailable(_))));
    }

    #[test]
    fn unexpected_http_status_is_provider_unavailable_not_passed() {
        let result = check_result_from_response(500, r#"{"success": true}"#);

        assert!(matches!(result, Err(CaptchaError::ProviderUnavailable(_))));
    }
}
