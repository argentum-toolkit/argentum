use secrecy::{ExposeSecret, SecretString};

use crate::password::{EncryptionError, Encryptor, Validator};

//TODO: implement better SALT
const SALT: &str = "encoded_";

pub struct EncryptorMock {}
pub struct ValidatorMock {}

impl EncryptorMock {
    pub fn new() -> EncryptorMock {
        EncryptorMock {}
    }
}

impl Default for EncryptorMock {
    fn default() -> Self {
        Self::new()
    }
}

impl Encryptor for EncryptorMock {
    fn encrypt(
        &self,
        password: &SecretString,
    ) -> Result<(SecretString, SecretString), EncryptionError> {
        let salt = SALT.to_string();
        let hash = [SALT, password.expose_secret()].join("");

        Ok((hash.into(), salt.into()))
    }
}

impl ValidatorMock {
    pub fn new() -> ValidatorMock {
        ValidatorMock {}
    }
}

impl Default for ValidatorMock {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator for ValidatorMock {
    fn validate(
        &self,
        password: &SecretString,
        salt: &SecretString,
        encoded_password: &SecretString,
    ) -> Result<bool, EncryptionError> {
        let hash = [salt.expose_secret(), password.expose_secret()].join("");

        Ok(encoded_password.expose_secret() == hash)
    }
}
