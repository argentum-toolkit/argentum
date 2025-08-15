use crate::invariant_violation::InvariantResult;
use compiletime_regex::regex;
use std::sync::LazyLock;

const ERR_EMAIL_EMPTY: &str = "Email should not be empty";
const ERR_WRONG_EMAIL: &str = "Wrong email address";

pub static EMAIL_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex!(r"^([\w\.\-]+)@([\w\-]+)((\.(\w){2,10})+)$"));

#[derive(Clone, PartialEq)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn try_new(email: &str) -> InvariantResult<EmailAddress> {
        if email.is_empty() {
            //Constant will be converted into `Violation`
            return Err(ERR_EMAIL_EMPTY.into());
        }

        if EMAIL_REGEX.is_match(email) {
            Ok(EmailAddress(email.into()))
        } else {
            //Constant will be converted into `Violation`
            Err(ERR_WRONG_EMAIL.into())
        }
    }

    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::email::{ERR_EMAIL_EMPTY, ERR_WRONG_EMAIL, EmailAddress};

    #[test]
    fn test_new_valid_email_address() {
        let email_str = "man@example.com";
        let res = EmailAddress::try_new(&email_str);

        match res {
            Ok(email) => assert_eq!(email_str.to_string(), email.as_string()),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_new_empty_email_address() {
        let res = EmailAddress::try_new("");

        assert!(res.is_err());

        if let Err(violations) = res {
            assert!(violations.items.is_none());
            assert_eq!(violations.errors.len(), 1);
            let v = violations.errors.first().expect("Should be not empty");
            assert_eq!(v, ERR_EMAIL_EMPTY)
        }
    }

    #[test]
    fn test_new_wrong_email_address() {
        let res = EmailAddress::try_new("a@aa");

        assert!(res.is_err());

        if let Err(violations) = res {
            assert!(violations.items.is_none());
            assert_eq!(violations.errors.len(), 1);
            let v = violations.errors.first().expect("Should be not empty");
            assert_eq!(v, ERR_WRONG_EMAIL)
        }
    }
}
