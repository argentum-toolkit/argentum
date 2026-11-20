use std::sync::Arc;

use argentum_captcha_business::service::CaptchaProviderTrait;
use argentum_standard_business::di_container::Container;
use secrecy::SecretString;

use crate::TurnstileCaptchaProvider;

pub fn captcha_infrastructure_provide_turnstile(
    container: &mut Container,
    secret_key: SecretString,
    client: Arc<reqwest::Client>,
) {
    let provider = Arc::new(TurnstileCaptchaProvider::new(secret_key, client));

    container.provide_factory::<dyn CaptchaProviderTrait, _>(move |_c| Ok(provider.clone()));
}
