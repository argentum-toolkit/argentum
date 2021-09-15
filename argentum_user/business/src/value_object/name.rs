#[derive(thiserror::Error, Debug)]
pub enum WrongNameError {
    #[error("First Name should not be empty")]
    FirstNameEmpty,

    #[error("Last Name should not be empty")]
    LastNameEmpty,
}

pub struct Name {
    pub first: String,
    pub last: String,
}

impl Name {
    pub fn new(first: String, last: String) -> Result<Name, WrongNameError> {
        if first.is_empty() {
            return Err(WrongNameError::FirstNameEmpty);
        }

        if last.is_empty() {
            return Err(WrongNameError::LastNameEmpty);
        }

        Ok(Name { first, last })
    }
}

impl Clone for Name {
    fn clone(&self) -> Name {
        Name {
            first: self.first.clone(),
            last: self.last.clone(),
        }
    }
}
