use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use async_trait::async_trait;

use crate::{CaptchaError, data_type::Captcha, service::CaptchaProviderTrait};

#[async_trait]
pub trait UserPassesCaptchaUcTrait: Send + Sync {
    async fn execute(&self, captcha: Option<&Captcha>) -> Result<bool, CaptchaError>;
}

pub struct UserPassesCaptchaUc<L>
where
    L: LoggerTrait,
{
    provider: Arc<dyn CaptchaProviderTrait>,
    captcha_enabled: bool,
    logger: Arc<L>,
}

impl<L> UserPassesCaptchaUc<L>
where
    L: LoggerTrait,
{
    pub fn new(
        provider: Arc<dyn CaptchaProviderTrait>,
        captcha_enabled: bool,
        logger: Arc<L>,
    ) -> Self {
        Self {
            provider,
            captcha_enabled,
            logger,
        }
    }

    pub async fn execute(&self, captcha: Option<&Captcha>) -> Result<bool, CaptchaError> {
        if !self.captcha_enabled {
            self.logger.warning("Captcha check is disabled");

            return Ok(true);
        }

        let Some(captcha) = captcha else {
            return Ok(false);
        };

        self.provider.verify(captcha).await
    }
}

#[async_trait]
impl<L> UserPassesCaptchaUcTrait for UserPassesCaptchaUc<L>
where
    L: LoggerTrait,
{
    async fn execute(&self, captcha: Option<&Captcha>) -> Result<bool, CaptchaError> {
        UserPassesCaptchaUc::execute(self, captcha).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    };

    use argentum_log_business::{Level, LoggerTrait};
    use async_trait::async_trait;

    use crate::{CaptchaError, data_type::Captcha, service::CaptchaProviderTrait};

    use super::UserPassesCaptchaUc;

    struct StubProvider {
        answer: Option<bool>,
        calls: AtomicUsize,
    }

    impl StubProvider {
        fn answering(answer: Option<bool>) -> Arc<Self> {
            Arc::new(Self {
                answer,
                calls: AtomicUsize::new(0),
            })
        }

        fn calls(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl CaptchaProviderTrait for StubProvider {
        async fn verify(&self, _captcha: &Captcha) -> Result<bool, CaptchaError> {
            self.calls.fetch_add(1, Ordering::SeqCst);

            self.answer
                .ok_or_else(|| CaptchaError::ProviderUnavailable("stub is down".to_string()))
        }
    }

    #[derive(Default)]
    struct RecordingLogger {
        records: Mutex<Vec<(Level, String)>>,
    }

    impl LoggerTrait for RecordingLogger {
        fn log<S: AsRef<str>>(&self, level: Level, msg: S) {
            self.records
                .lock()
                .expect("logger lock should not be poisoned")
                .push((level, msg.as_ref().to_string()));
        }
    }

    fn token() -> Result<Captcha, String> {
        Captcha::try_new("token-value").map_err(|error| format!("{error:?}"))
    }

    #[tokio::test]
    async fn passes_when_provider_passes() -> Result<(), String> {
        let provider = StubProvider::answering(Some(true));
        let uc =
            UserPassesCaptchaUc::new(provider.clone(), true, Arc::new(RecordingLogger::default()));

        let result = uc
            .execute(Some(&token()?))
            .await
            .map_err(|error| format!("{error:?}"))?;

        assert!(result);
        assert_eq!(provider.calls(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn fails_when_provider_fails() -> Result<(), String> {
        let provider = StubProvider::answering(Some(false));
        let uc =
            UserPassesCaptchaUc::new(provider.clone(), true, Arc::new(RecordingLogger::default()));

        let result = uc
            .execute(Some(&token()?))
            .await
            .map_err(|error| format!("{error:?}"))?;

        assert!(!result);
        assert_eq!(provider.calls(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn returns_error_when_provider_is_unavailable() -> Result<(), String> {
        let provider = StubProvider::answering(None);
        let uc =
            UserPassesCaptchaUc::new(provider.clone(), true, Arc::new(RecordingLogger::default()));

        let result = uc.execute(Some(&token()?)).await;

        assert!(matches!(result, Err(CaptchaError::ProviderUnavailable(_))));
        assert_eq!(provider.calls(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn enabled_without_captcha_fails_without_calling_provider() -> Result<(), String> {
        let provider = StubProvider::answering(Some(true));
        let uc =
            UserPassesCaptchaUc::new(provider.clone(), true, Arc::new(RecordingLogger::default()));

        let result = uc
            .execute(None)
            .await
            .map_err(|error| format!("{error:?}"))?;

        assert!(!result);
        assert_eq!(provider.calls(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn disabled_passes_without_calling_provider_and_logs_it() -> Result<(), String> {
        let provider = StubProvider::answering(None);
        let logger = Arc::new(RecordingLogger::default());
        let uc = UserPassesCaptchaUc::new(provider.clone(), false, logger.clone());

        let result = uc
            .execute(None)
            .await
            .map_err(|error| format!("{error:?}"))?;

        assert!(result);
        assert_eq!(provider.calls(), 0);

        let records = logger
            .records
            .lock()
            .map_err(|error| format!("{error:?}"))?;
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].0, Level::Warning);
        Ok(())
    }
}
