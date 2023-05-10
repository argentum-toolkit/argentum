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
    fn encrypt(&self, password: &str) -> Result<(String, String), EncryptionError> {
        let salt = SALT.to_string();
        let hash = [SALT, password].join("");

        Ok((hash, salt))
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
    fn validate(&self, password: &str, salt: &str, encoded_password: &str) -> bool {
        let hash = [salt, password].join("");

        encoded_password == hash
    }
}
