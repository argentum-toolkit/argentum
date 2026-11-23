use argentum_captcha_business::{
    CaptchaError, data_type::Captcha, use_case::UserPassesCaptchaUcTrait,
};
use async_trait::async_trait;

pub struct UserPassesCaptchaMock {
    answer: Option<bool>,
}

impl UserPassesCaptchaMock {
    pub fn passing() -> Self {
        Self { answer: Some(true) }
    }

    pub fn failing() -> Self {
        Self {
            answer: Some(false),
        }
    }

    pub fn unavailable() -> Self {
        Self { answer: None }
    }
}

#[async_trait]
impl UserPassesCaptchaUcTrait for UserPassesCaptchaMock {
    async fn execute(&self, _captcha: Option<&Captcha>) -> Result<bool, CaptchaError> {
        self.answer
            .ok_or_else(|| CaptchaError::ProviderUnavailable("mock is down".to_string()))
    }
}
