use argentum_standard_business::invariant_violation::InvariantResult;
use std::fmt::Display;

const ERR_NAME_EMPTY: &str = "Should not be empty";

#[derive(Clone, Debug)]
pub struct NamePart(String);

impl NamePart {
    pub fn try_new(value: String) -> InvariantResult<Self> {
        if value.is_empty() {
            return Err(ERR_NAME_EMPTY.into());
        }

        Ok(Self(value))
    }
}

impl Display for NamePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

#[cfg(test)]
mod test {
    use crate::data_type::name_part::ERR_NAME_EMPTY;
    use crate::data_type::NamePart;

    #[test]
    fn test_new_valid() {
        let res = NamePart::try_new("Andrey".into());
        assert!(res.is_ok());
        assert_eq!(res.unwrap().0, "Andrey");
    }

    #[test]
    fn test_new_empty() {
        let res = NamePart::try_new("".into());
        assert!(res.is_err());

        let violations = res.unwrap_err();

        assert!(violations.items.is_none());
        assert_eq!(violations.errors.len(), 1);

        let v = violations.errors.first().unwrap();
        assert_eq!(v, &ERR_NAME_EMPTY.to_string());
    }
}
