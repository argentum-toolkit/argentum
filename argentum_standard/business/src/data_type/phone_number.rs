use crate::invariant_violation::{InvariantResult, Violations};

#[derive(Clone, PartialEq, Debug)]
pub struct PhoneNumber {
    pub country_code: u32,
    pub number: u64,
}

impl PhoneNumber {
    pub fn try_new(country_code: u32, number: u64) -> InvariantResult<PhoneNumber> {
        let full_number = format!("+{}{}", country_code, number);
        match phonenumber::parse(None, &full_number) {
            Ok(_) => Ok(Self {
                country_code,
                number,
            }),
            Err(_e) => Err(Violations::new(vec!["wrong phone number".into()], None)),
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
