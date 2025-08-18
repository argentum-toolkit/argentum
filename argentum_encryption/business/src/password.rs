pub trait Encryptor: Send + Sync {
    fn encrypt(&self, password: &str) -> Result<(String, String), EncryptionError>;
}

pub trait Validator: Send + Sync {
    fn validate(
        &self,
        password: &str,
        salt: &str,
        encoded_password: &str,
    ) -> Result<bool, EncryptionError>;
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum EncryptionError {
    #[error("Can't generate a salt")]
    SaltError,

    #[error("Encription error: {0}")]
    Other(String),
}
