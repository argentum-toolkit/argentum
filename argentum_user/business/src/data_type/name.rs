#[derive(thiserror::Error, Debug)]
pub enum WrongNameError {
    #[error("First Name should not be empty")]
    FirstNameEmpty,

    #[error("Last Name should not be empty")]
    LastNameEmpty,

    #[error("PatronymicEmpty should be none or not empty string")]
    PatronymicEmpty,
}

pub struct Name {
    pub first: String,
    pub last: Option<String>,
    pub patronymic: Option<String>,
}

impl Name {
    pub fn new(
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

impl Clone for Name {
    fn clone(&self) -> Name {
        Name {
            first: self.first.clone(),
            last: self.last.clone(),
            patronymic: self.patronymic.clone(),
        }
    }
}
