use std::collections::BTreeMap;

use crate::invariant_violation::{InvariantResult, Violations};

#[derive(Clone, PartialEq, Debug)]
pub struct PhoneNumber {
    pub country_code: u32,
    pub number: u64,
}

impl PhoneNumber {
    pub fn try_new(country_code: i32, local_number: i64) -> InvariantResult<PhoneNumber> {
        let mut vo = BTreeMap::new();

        let code = match u32::try_from(country_code) {
            Ok(c) => Some(c),
            Err(_) => {
                vo.insert(
                    "country_code",
                    Violations::new(vec!["country code should be more than 0".into()], None),
                );

                None
            }
        };

        let number = match u64::try_from(local_number) {
            Ok(c) => Some(c),
            Err(_) => {
                vo.insert(
                    "number",
                    Violations::new(vec!["number should be more than 0".into()], None),
                );

                None
            }
        };

        if vo.is_empty()
            && let Some(c) = code
            && let Some(n) = number
        {
            let full_number = format!("+{}{}", c, n);

            match phonenumber::parse(None, &full_number) {
                Ok(_) => Ok(Self {
                    country_code: c,
                    number: n,
                }),
                Err(_e) => Err(Violations::new(vec!["wrong phone number".into()], None)),
            }
        } else {
            Err(Violations::new(vec!["wrong phone number".into()], None))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phone_number() {
        let result = PhoneNumber::try_new(1, 1234567890);
        assert!(result.is_ok());
        let phone = result.unwrap();
        assert_eq!(phone.country_code, 1);
        assert_eq!(phone.number, 1234567890);
    }

    #[test]
    fn test_invalid_phone_number() {
        assert!(PhoneNumber::try_new(999, 1234567890).is_err());
        assert!(PhoneNumber::try_new(1, 99999999999999999).is_err());
    }
}
