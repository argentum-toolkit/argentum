use async_trait::async_trait;

use crate::{CaptchaError, data_type::Captcha, service::CaptchaProviderTrait};

#[derive(Default)]
pub struct AlwaysPassingCaptchaProviderMock {}

impl AlwaysPassingCaptchaProviderMock {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CaptchaProviderTrait for AlwaysPassingCaptchaProviderMock {
    async fn verify(&self, _captcha: &Captcha) -> Result<bool, CaptchaError> {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::AlwaysPassingCaptchaProviderMock;
    use crate::{data_type::Captcha, service::CaptchaProviderTrait};

    #[tokio::test]
    async fn always_passes() -> Result<(), String> {
        let captcha = Captcha::try_new("any-token").map_err(|error| format!("{error:?}"))?;

        let result = AlwaysPassingCaptchaProviderMock::new()
            .verify(&captcha)
            .await;

        assert!(matches!(result, Ok(true)));
        Ok(())
    }
}
