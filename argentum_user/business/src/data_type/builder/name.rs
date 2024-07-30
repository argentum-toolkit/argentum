use crate::data_type::{Name, NamePart};
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;
use std::vec;

const ERR_WRONG_NAME: &str = "Wrong name";

pub struct NameBuilder {
    first: Option<NamePart>,
    last: Option<NamePart>,
    patronymic: Option<NamePart>,
    violations: ViolationObject,
}

impl NameBuilder {
    pub fn new(first: String) -> Self {
        let mut violations = BTreeMap::new();

        let first_name = match NamePart::try_new(first) {
            Ok(f) => Some(f),
            Err(v) => {
                violations.insert("first".into(), v);
                None
            }
        };

        Self {
            first: first_name,
            last: None,
            patronymic: None,
            violations,
        }
    }

    pub fn last(mut self, last: Option<String>) -> Self {
        self.last = match last {
            None => None,
            Some(l) => match NamePart::try_new(l) {
                Ok(ln) => Some(ln),
                Err(v) => {
                    self.violations.insert("last".into(), v);

                    None
                }
            },
        };

        self
    }

    pub fn patronymic(mut self, patronymic: Option<String>) -> Self {
        self.patronymic = match patronymic {
            None => None,
            Some(l) => match NamePart::try_new(l) {
                Ok(ln) => Some(ln),
                Err(v) => {
                    self.violations.insert("patronymic".into(), v);

                    None
                }
            },
        };

        self
    }

    pub fn try_build(&self) -> InvariantResult<Name> {
        if self.violations.is_empty() {
            Ok(Name::new(
                self.first.clone().unwrap(),
                self.last.clone(),
                self.patronymic.clone(),
            ))
        } else {
            Err(Violations::new(
                vec![ERR_WRONG_NAME.to_string()],
                Some(ViolationItem::Object(self.violations.clone())),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::builder::name::ERR_WRONG_NAME;
    use crate::data_type::builder::NameBuilder;
    use argentum_standard_business::invariant_violation::ViolationItem;

    #[test]
    fn test_new_full() {
        let first = "First".to_string();
        let last = Some("Last".into());
        let patronymic = Some("Patronymic".into());

        let builder = NameBuilder::new(first.clone());
        let res = builder
            .last(last.clone())
            .patronymic(patronymic.clone())
            .try_build();

        let name = res.unwrap();

        assert_eq!(name.first.to_string(), first);
        assert_eq!(name.last.unwrap().to_string(), last.unwrap());
        assert_eq!(name.patronymic.unwrap().to_string(), patronymic.unwrap());
    }

    #[test]
    fn test_new_minimal() {
        let first = "First".to_string();
        let last = Some("Last".into());
        let patronymic = Some("Patronymic".into());

        let builder = NameBuilder::new(first.clone());
        let res = builder
            .last(last.clone())
            .patronymic(patronymic.clone())
            .try_build();

        let name = res.unwrap();

        assert_eq!(name.first.to_string(), first);
        assert_eq!(name.last.unwrap().to_string(), last.unwrap());
        assert_eq!(name.patronymic.unwrap().to_string(), patronymic.unwrap());
    }

    #[test]
    fn test_new_error_for_empty_fields() {
        let first = "".to_string();
        let last = Some("".into());
        let patronymic = Some("".into());
        let builder = NameBuilder::new(first.clone());
        let res = builder
            .last(last.clone())
            .patronymic(patronymic.clone())
            .try_build();

        assert!(res.is_err());

        if let Err(violations) = res {
            assert_eq!(violations.errors.len(), 1);
            assert_eq!(
                violations.errors.first().unwrap(),
                &ERR_WRONG_NAME.to_string()
            );
            assert!(violations.items.is_some());
            let items = violations.items.unwrap();

            assert!(!items.is_empty());

            if let ViolationItem::Object(v) = items {
                let f_errors = &v.get("first").unwrap().errors;
                assert_eq!(f_errors.len(), 1);
                assert_eq!(
                    f_errors.first().unwrap(),
                    &"Should not be empty".to_string()
                );

                let l_errors = &v.get("last").unwrap().errors;
                assert_eq!(l_errors.len(), 1);
                assert_eq!(
                    l_errors.first().unwrap(),
                    &"Should not be empty".to_string()
                );

                let p_errors = &v.get("first").unwrap().errors;
                assert_eq!(p_errors.len(), 1);
                assert_eq!(
                    p_errors.first().unwrap(),
                    &"Should not be empty".to_string()
                );
            }
        }
    }
}
