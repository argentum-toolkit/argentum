use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WrongEmailError {
    #[error("Email should not be empty")]
    Empty,
    #[error("Wrong email address")]
    WrongEmail,
}

pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(email: String) -> Result<EmailAddress, WrongEmailError> {
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

impl PartialEq for EmailAddress {
    fn eq(&self, other: &Self) -> bool {
        other.0 == self.0
    }
}

impl Clone for EmailAddress {
    fn clone(&self) -> EmailAddress {
        EmailAddress(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::email::EmailAddress;

    #[test]
    fn test_new_valid_email_address() {
        let email_string = String::from("man@example.com");
        let res = EmailAddress::new(email_string.clone());

        match res {
            Ok(email) => {
                println!("{}", email_string);
                println!("{}", email.as_string());
                assert_eq!(email_string, email.as_string())
            }
            Err(_) => {
                assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn test_new_wrong_email_address() {
        let email_string = String::from("a@aa");
        let res = EmailAddress::new(email_string.clone());

        match res {
            Ok(_) => {
                assert_eq!(true, false)
            }
            Err(_) => {
                assert_eq!(true, true)
            }
        }
    }
}
