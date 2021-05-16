pub trait Encryptor {
    fn encrypt(&self, password: &str) -> Result<(String, String), EncryptionError>;
}

pub trait Validator {
    fn validate(&self, password: &str, salt: &str, encoded_password: &str) -> bool;
}

#[derive(thiserror::Error, Debug)]
pub enum EncryptionError {
    #[error("Can't generate a salt")]
    SaltError,
}
