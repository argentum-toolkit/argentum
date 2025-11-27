use secrecy::SecretString;

pub trait Encryptor: Send + Sync {
    /// Encrypts the password and returns the hash and the salt used to encrypt it.
    /// @return (hash, salt)
    fn encrypt(
        &self,
        password: &SecretString,
    ) -> Result<(SecretString, SecretString), EncryptionError>;
}

pub trait Validator: Send + Sync {
    fn validate(
        &self,
        password: &SecretString,
        salt: &SecretString,
        encoded_password: &SecretString,
    ) -> Result<bool, EncryptionError>;
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum EncryptionError {
    #[error("Can't generate a salt")]
    SaltError,

    #[error("Encription error: {0}")]
    Other(String),
}
