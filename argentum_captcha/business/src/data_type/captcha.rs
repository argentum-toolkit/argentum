use argentum_standard_business::invariant_violation::InvariantResult;

const ERR_CAPTCHA_EMPTY: &str = "Captcha should not be empty";
const ERR_CAPTCHA_TOO_LONG: &str = "Captcha is too long";

const CAPTCHA_MAX_LENGTH: usize = 2048;

#[derive(Clone, PartialEq, Debug)]
pub struct Captcha(String);

impl Captcha {
    pub fn try_new(token: &str) -> InvariantResult<Self> {
        if token.is_empty() {
            return Err(ERR_CAPTCHA_EMPTY.into());
        }

        if token.len() > CAPTCHA_MAX_LENGTH {
            return Err(ERR_CAPTCHA_TOO_LONG.into());
        }

        Ok(Captcha(token.into()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::Captcha;
    use crate::data_type::captcha::{CAPTCHA_MAX_LENGTH, ERR_CAPTCHA_EMPTY, ERR_CAPTCHA_TOO_LONG};

    #[test]
    fn creates_captcha_from_non_empty_token() {
        let captcha = Captcha::try_new("token-value").expect("token should be valid");

        assert_eq!(captcha.as_str(), "token-value");
    }

    #[test]
    fn accepts_token_at_max_length() {
        let token = "a".repeat(CAPTCHA_MAX_LENGTH);

        assert!(Captcha::try_new(&token).is_ok());
    }

    #[test]
    fn rejects_empty_token() {
        let violations = Captcha::try_new("").expect_err("empty token should be rejected");

        assert!(violations.items.is_none());
        assert_eq!(violations.errors, vec![ERR_CAPTCHA_EMPTY.to_string()]);
    }

    #[test]
    fn rejects_token_longer_than_max_length() {
        let token = "a".repeat(CAPTCHA_MAX_LENGTH + 1);

        let violations = Captcha::try_new(&token).expect_err("long token should be rejected");

        assert!(violations.items.is_none());
        assert_eq!(violations.errors, vec![ERR_CAPTCHA_TOO_LONG.to_string()]);
    }
}
