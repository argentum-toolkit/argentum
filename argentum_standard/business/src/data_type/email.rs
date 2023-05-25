use crate::invariant_violation::InvariantResult;
use regex::Regex;

const ERR_EMAIL_EMPTY: &str = "Email should not be empty";
const ERR_WRONG_EMAIL: &str = "Wrong email address";

#[derive(Clone, PartialEq)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn try_new(email: String) -> InvariantResult<EmailAddress> {
        if email.is_empty() {
            //Constant will be converted into `Violation`
            return Err(ERR_EMAIL_EMPTY.into());
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
            //Constant will be converted into `Violation`
            Err(ERR_WRONG_EMAIL.into())
        }
    }

    //TODO: to_string
    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::email::{EmailAddress, ERR_EMAIL_EMPTY, ERR_WRONG_EMAIL};

    #[test]
    fn test_new_valid_email_address() {
        let email_string = "man@example.com".to_string();
        let res = EmailAddress::try_new(email_string.clone());

        match res {
            Ok(email) => assert_eq!(email_string, email.as_string()),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_new_empty_email_address() {
        let res = EmailAddress::try_new("".into());

        assert!(res.is_err());

        if let Err(violations) = res {
            assert!(violations.items.is_none());
            assert_eq!(violations.errors.len(), 1);
            let v = violations.errors.first().unwrap();
            assert_eq!(v, ERR_EMAIL_EMPTY)
        }
    }

    #[test]
    fn test_new_wrong_email_address() {
        let res = EmailAddress::try_new("a@aa".into());

        assert!(res.is_err());

        if let Err(violations) = res {
            assert!(violations.items.is_none());
            assert_eq!(violations.errors.len(), 1);
            let v = violations.errors.first().unwrap();
            assert_eq!(v, ERR_WRONG_EMAIL)
        }
    }
}
