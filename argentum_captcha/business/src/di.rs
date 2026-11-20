use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_standard_business::di_container::{Container, DiError};

use crate::{
    mock::AlwaysPassingCaptchaProviderMock,
    service::CaptchaProviderTrait,
    use_case::{UserPassesCaptchaUc, UserPassesCaptchaUcTrait},
};

pub fn captcha_business_provide<L>(container: &mut Container, captcha_enabled: bool)
where
    L: LoggerTrait + 'static,
{
    container.provide_factory::<UserPassesCaptchaUc<L>, _>(move |c| {
        let provider = c.resolve::<dyn CaptchaProviderTrait>()?;
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(UserPassesCaptchaUc::new(
            provider,
            captcha_enabled,
            logger,
        )))
    });

    container.provide_factory::<dyn UserPassesCaptchaUcTrait, _>(|c| {
        let uc = c.resolve::<UserPassesCaptchaUc<L>>()?;

        Ok(uc)
    });
}

pub fn captcha_business_provide_mock(container: &mut Container) {
    container.provide_factory::<dyn CaptchaProviderTrait, _>(|_c| {
        Ok(Arc::new(AlwaysPassingCaptchaProviderMock::new()))
    });
}

pub fn resolve_captcha_business<L>(
    container: &Container,
) -> Result<Arc<UserPassesCaptchaUc<L>>, DiError>
where
    L: LoggerTrait + 'static,
{
    container.resolve::<UserPassesCaptchaUc<L>>()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use argentum_log_business::Level;
    use argentum_log_business::LoggerTrait;
    use argentum_standard_business::di_container::Container;

    use crate::{data_type::Captcha, use_case::UserPassesCaptchaUcTrait};

    use super::{
        captcha_business_provide, captcha_business_provide_mock, resolve_captcha_business,
    };

    struct SilentLogger;

    impl LoggerTrait for SilentLogger {
        fn log<S: AsRef<str>>(&self, _level: Level, _msg: S) {}
    }

    #[tokio::test]
    async fn resolves_use_case_wired_with_mock_provider() -> Result<(), String> {
        let mut container = Container::new();
        container.provide_factory::<SilentLogger, _>(|_c| Ok(Arc::new(SilentLogger)));
        captcha_business_provide_mock(&mut container);
        captcha_business_provide::<SilentLogger>(&mut container, true);

        let uc = resolve_captcha_business::<SilentLogger>(&container)
            .map_err(|error| format!("{error:?}"))?;
        let captcha = Captcha::try_new("token-value").map_err(|error| format!("{error:?}"))?;

        let result = uc
            .execute(Some(&captcha))
            .await
            .map_err(|error| format!("{error:?}"))?;

        assert!(result);
        Ok(())
    }

    #[tokio::test]
    async fn resolves_dyn_trait_use_case() -> Result<(), String> {
        let mut container = Container::new();
        container.provide_factory::<SilentLogger, _>(|_c| Ok(Arc::new(SilentLogger)));
        captcha_business_provide_mock(&mut container);
        captcha_business_provide::<SilentLogger>(&mut container, true);

        let uc = container
            .resolve::<dyn UserPassesCaptchaUcTrait>()
            .map_err(|error| format!("{error:?}"))?;

        let result = uc
            .execute(None)
            .await
            .map_err(|error| format!("{error:?}"))?;

        assert!(!result);
        Ok(())
    }
}
