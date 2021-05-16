use crate::password::{EncryptionError, Encryptor, Validator};

const SALT: &str = "encoded_";

pub struct EncryptorMock {}
pub struct ValidatorMock {}

impl EncryptorMock {
    pub fn new() -> EncryptorMock {
        EncryptorMock {}
    }
}

impl Encryptor for EncryptorMock {
    fn encrypt(&self, password: &str) -> Result<(String, String), EncryptionError> {
        let salt = String::from(SALT);
        let hash = [SALT, password].join("");

        Ok((hash, salt))
    }
}

impl ValidatorMock {
    pub fn new() -> ValidatorMock {
        ValidatorMock {}
    }
}

impl Validator for ValidatorMock {
    fn validate(&self, password: &str, salt: &str, encoded_password: &str) -> bool {
        let hash = [salt, password].join("");

        encoded_password == hash
    }
}
