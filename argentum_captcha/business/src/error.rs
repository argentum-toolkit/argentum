#[derive(thiserror::Error, Debug)]
pub enum CaptchaError {
    #[error("Captcha can't be checked. Provider is unavailable: `{0}`")]
    ProviderUnavailable(String),
}
