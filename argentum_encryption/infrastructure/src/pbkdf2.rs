use argentum_encryption_business::password::{EncryptionError, Encryptor, Validator};
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use secrecy::{ExposeSecret, SecretString};
use std::num::NonZeroU32;

pub struct Pbkdf2 {}

impl Pbkdf2 {
    pub fn new() -> Pbkdf2 {
        Pbkdf2 {}
    }
}

impl Default for Pbkdf2 {
    fn default() -> Self {
        Self::new()
    }
}

const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;

impl Encryptor for Pbkdf2 {
    fn encrypt(
        &self,
        password: &SecretString,
    ) -> Result<(SecretString, SecretString), EncryptionError> {
        let n_iter: NonZeroU32 = NonZeroU32::new(100000)
            .ok_or_else(|| EncryptionError::Other("Can't create NonZeroU32".into()))?;
        let rng = rand::SystemRandom::new();

        let mut salt = [0u8; CREDENTIAL_LEN];
        if let Err(_err) = rng.fill(&mut salt) {
            //TODO: cover error
            //TODO: log err.to_string());
            return Err(EncryptionError::SaltError);
        }

        let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            password.expose_secret().as_bytes(),
            &mut pbkdf2_hash,
        );

        let hash = HEXUPPER.encode(&pbkdf2_hash);
        let salt_str = HEXUPPER.encode(&salt);

        Ok((hash.into(), salt_str.into()))
    }
}

impl Validator for Pbkdf2 {
    fn validate(
        &self,
        password: &SecretString,
        salt: &SecretString,
        encoded_password: &SecretString,
    ) -> Result<bool, EncryptionError> {
        let n_iter: NonZeroU32 = NonZeroU32::new(100000)
            .ok_or_else(|| EncryptionError::Other("Can't create NonZeroU32".into()))?;

        let should_succeed = pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &HEXUPPER
                .decode(salt.expose_secret().as_bytes())
                .map_err(|e| EncryptionError::Other(e.to_string()))?,
            password.expose_secret().as_bytes(),
            &HEXUPPER
                .decode(encoded_password.expose_secret().as_bytes())
                .map_err(|e| EncryptionError::Other(e.to_string()))?,
        );

        match should_succeed {
            Ok(()) => Ok(true),
            Err(..) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pbkdf2::Pbkdf2;
    use argentum_encryption_business::password::{Encryptor, Validator};
    use secrecy::ExposeSecret;
    use secrecy::SecretString;

    fn stub_password() -> SecretString {
        "123456".into()
    }

    fn stub_hash_and_salt() -> (SecretString, SecretString) {
        (
            "14BF98B8522FAF0A4ED6431A2FC48168106BDA4243C64AAC641CBA32FD81C97\
            5D7CACA60DA051DC5596DB8BA461E22F3C069DF7AEEECF80576F163CEC287F16A"
                .into(),
            "14565CCF70BA588B714364A2813A1A489E62FE32FC6871F53A5B909F2D1C987\
            91F57CA9F27A5DC94526B43FCCF0F25516BC515FAF32BFC0CEDD2B34A8222F5AA"
                .into(),
        )
    }

    #[test]
    fn test_pbkdf2_encrypt() {
        let password = stub_password();
        let pbkdf2 = Pbkdf2::new();
        let result = pbkdf2.encrypt(&password);

        assert_eq!(true, result.is_ok());

        match result {
            Ok((hash, salt)) => {
                assert_eq!(128, hash.expose_secret().len());
                assert_eq!(128, salt.expose_secret().len());
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_pbkdf2_validate_correct_password() {
        let password = stub_password();
        let (hash, salt) = stub_hash_and_salt();

        let pbkdf2 = Pbkdf2::new();
        let result = pbkdf2.validate(&password, &salt, &hash);

        assert_eq!(Ok(true), result);
    }

    #[test]
    fn test_pbkdf2_validate_wrong_password() {
        let password = "111111";
        let (hash, salt) = stub_hash_and_salt();

        let pbkdf2 = Pbkdf2::new();
        let result = pbkdf2.validate(&password.into(), &salt, &hash);

        assert_eq!(Ok(false), result);
    }
}
