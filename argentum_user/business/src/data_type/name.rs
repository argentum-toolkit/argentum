#[derive(thiserror::Error, Debug)]
pub enum WrongNameError {
    #[error("First Name should not be empty")]
    FirstNameEmpty,

    #[error("Last Name should not be empty")]
    LastNameEmpty,

    #[error("PatronymicEmpty should be none or not empty string")]
    PatronymicEmpty,
}

#[derive(Clone)]
pub struct Name {
    pub first: String,
    pub last: Option<String>,
    pub patronymic: Option<String>,
}

impl Name {
    pub fn try_new(
        first: String,
        last: Option<String>,
        patronymic: Option<String>,
    ) -> Result<Name, WrongNameError> {
        if first.is_empty() {
            return Err(WrongNameError::FirstNameEmpty);
        }

        if last.is_some() && last.clone().unwrap().is_empty() {
            return Err(WrongNameError::LastNameEmpty);
        }

        if patronymic.is_some() && patronymic.clone().unwrap().is_empty() {
            return Err(WrongNameError::PatronymicEmpty);
        }

        Ok(Name {
            first,
            last,
            patronymic,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::name::WrongNameError;
    use crate::data_type::Name;

    #[test]
    fn test_new_full() {
        let first = "First".to_string();
        let last = Some("Last".into());
        let patronymic = Some("Patronymic".into());
        let res = Name::try_new(first.clone(), last.clone(), patronymic.clone());

        let name = res.unwrap();

        assert_eq!(name.first, first);
        assert_eq!(name.last, last);
        assert_eq!(name.patronymic, patronymic);
    }

    #[test]
    fn test_new_minimal() {
        let first = "First".to_string();
        let res = Name::try_new(first.clone(), None, None);

        let name = res.unwrap();

        assert_eq!(name.first, first);
        assert_eq!(name.last, None);
        assert_eq!(name.patronymic, None);
    }

    #[test]
    fn test_new_error_for_empty_first_name() -> Result<(), &'static str> {
        let first = "".to_string();
        let last = Some("".into());
        let patronymic = Some("".into());
        let res = Name::try_new(first.clone(), last.clone(), patronymic.clone());

        match res {
            Ok(_) => Err("`try_new` Should return an error"),
            Err(e) => match e {
                WrongNameError::FirstNameEmpty => Ok(()),
                WrongNameError::LastNameEmpty => Err("Wrong error type"),
                WrongNameError::PatronymicEmpty => Err("Wrong error type"),
            },
        }
    }
    #[test]
    fn test_new_error_for_empty_last_name() -> Result<(), &'static str> {
        let first = "First".to_string();
        let last = Some("".into());
        let patronymic = Some("".into());
        let res = Name::try_new(first.clone(), last.clone(), patronymic.clone());

        match res {
            Ok(_) => Err("`try_new` Should return an error"),
            Err(WrongNameError::FirstNameEmpty) => Err("Wrong error type"),
            Err(WrongNameError::LastNameEmpty) => Ok(()),
            Err(WrongNameError::PatronymicEmpty) => Err("Wrong error type"),
        }
    }

    #[test]
    fn test_new_error_for_empty_patronymic_name() -> Result<(), &'static str> {
        let first = "First".to_string();
        let last = Some("Second".into());
        let patronymic = Some("".into());
        let res = Name::try_new(first.clone(), last.clone(), patronymic.clone());

        match res {
            Ok(_) => Err("`try_new` Should return an error"),
            Err(WrongNameError::FirstNameEmpty) => Err("Wrong error type"),
            Err(WrongNameError::LastNameEmpty) => Err("Wrong error type"),
            Err(WrongNameError::PatronymicEmpty) => Ok(()),
        }
    }
}
