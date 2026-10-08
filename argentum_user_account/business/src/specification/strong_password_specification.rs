use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PasswordSpecificationError {
    #[error("Password must contain at least {minimum} characters")]
    TooShort { minimum: usize },

    #[error("Password must contain at least one lowercase character")]
    MissingLowercase,

    #[error("Password must contain at least one uppercase character")]
    MissingUppercase,

    #[error("Password must contain at least one digit")]
    MissingDigit,

    #[error("Password must contain at least one special character")]
    MissingSpecialCharacter,
}

pub struct StrongPasswordSpecification {
    minimum_length: usize,
    complexity_enabled: bool,
}

impl StrongPasswordSpecification {
    pub fn new(minimum_length: usize, complexity_enabled: bool) -> Self {
        Self {
            minimum_length,
            complexity_enabled,
        }
    }

    pub fn is_satisfied_by(
        &self,
        password: &SecretString,
    ) -> Result<(), PasswordSpecificationError> {
        let value = password.expose_secret();

        if value.chars().count() < self.minimum_length {
            return Err(PasswordSpecificationError::TooShort {
                minimum: self.minimum_length,
            });
        }

        if !self.complexity_enabled {
            return Ok(());
        }

        if !value
            .chars()
            .any(|character| character.is_ascii_lowercase())
        {
            return Err(PasswordSpecificationError::MissingLowercase);
        }
        if !value
            .chars()
            .any(|character| character.is_ascii_uppercase())
        {
            return Err(PasswordSpecificationError::MissingUppercase);
        }
        if !value.chars().any(|character| character.is_ascii_digit()) {
            return Err(PasswordSpecificationError::MissingDigit);
        }
        if !value
            .chars()
            .any(|character| !character.is_ascii_alphanumeric())
        {
            return Err(PasswordSpecificationError::MissingSpecialCharacter);
        }

        Ok(())
    }
}

impl Default for StrongPasswordSpecification {
    fn default() -> Self {
        Self::new(8, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn password(value: &str) -> SecretString {
        SecretString::from(value.to_string())
    }

    #[test]
    fn accepts_password_with_complexity() {
        let specification = StrongPasswordSpecification::new(8, true);

        assert!(specification.is_satisfied_by(&password("Secure-1")).is_ok());
    }

    #[test]
    fn rejects_short_password_before_complexity_checks() {
        let specification = StrongPasswordSpecification::new(8, true);

        assert_eq!(
            specification.is_satisfied_by(&password("Ab-1")),
            Err(PasswordSpecificationError::TooShort { minimum: 8 })
        );
    }

    #[test]
    fn allows_simple_password_when_complexity_is_disabled() {
        let specification = StrongPasswordSpecification::new(8, false);

        assert!(specification.is_satisfied_by(&password("12345678")).is_ok());
    }

    #[test]
    fn still_enforces_minimum_length_when_complexity_is_disabled() {
        let specification = StrongPasswordSpecification::new(8, false);

        assert_eq!(
            specification.is_satisfied_by(&password("123")),
            Err(PasswordSpecificationError::TooShort { minimum: 8 })
        );
    }

    #[test]
    fn rejects_each_missing_complexity_requirement() {
        let specification = StrongPasswordSpecification::new(8, true);

        assert_eq!(
            specification.is_satisfied_by(&password("12345678")),
            Err(PasswordSpecificationError::MissingLowercase)
        );
        assert_eq!(
            specification.is_satisfied_by(&password("abcdefg1")),
            Err(PasswordSpecificationError::MissingUppercase)
        );
        assert_eq!(
            specification.is_satisfied_by(&password("Abcdefgh")),
            Err(PasswordSpecificationError::MissingDigit)
        );
        assert_eq!(
            specification.is_satisfied_by(&password("Abcdefg1")),
            Err(PasswordSpecificationError::MissingSpecialCharacter)
        );
    }
}
