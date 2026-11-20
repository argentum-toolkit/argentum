use async_trait::async_trait;

use crate::{CaptchaError, data_type::Captcha};

#[async_trait]
pub trait CaptchaProviderTrait: Send + Sync {
    async fn verify(&self, captcha: &Captcha) -> Result<bool, CaptchaError>;
}
