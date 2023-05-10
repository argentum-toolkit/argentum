use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WrongEmailError {
    #[error("Email should not be empty")]
    Empty,
    #[error("Wrong email address")]
    WrongEmail,
}

#[derive(Clone, PartialEq)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn try_new(email: String) -> Result<EmailAddress, WrongEmailError> {
        if email.is_empty() {
            return Err(WrongEmailError::Empty);
        }

        let re = Regex::new(
            r"(?x) # enable insigificant whitespace mode
            ^([\w\.\-]+)@([\w\-]+)((\.(\w){2,10})+)$
        ",
        )
        .unwrap();

        if re.is_match(email.as_str()) {
            Ok(EmailAddress(email))
        } else {
            Err(WrongEmailError::WrongEmail)
        }
    }

    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::email::{EmailAddress, WrongEmailError};

    #[test]
    fn test_new_valid_email_address() {
        let email_string = "man@example.com".to_string();
        let res = EmailAddress::try_new(email_string.clone());

        match res {
            Ok(email) => {
                assert_eq!(email_string, email.as_string())
            }
            Err(_) => {
                assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn test_new_empty_email_address() -> Result<(), &'static str> {
        let res = EmailAddress::try_new("".into());

        match res {
            Ok(_) => Err("`try_new` should return an error"),
            Err(WrongEmailError::Empty) => Ok(()),
            Err(WrongEmailError::WrongEmail) => Err("Wrong error type"),
        }
    }

    #[test]
    fn test_new_wrong_email_address() -> Result<(), &'static str> {
        let res = EmailAddress::try_new("a@aa".into());

        match res {
            Ok(_) => Err("`try_new` should return an error"),
            Err(WrongEmailError::Empty) => Err("Wrong error type"),
            Err(WrongEmailError::WrongEmail) => Ok(()),
        }
    }
}
