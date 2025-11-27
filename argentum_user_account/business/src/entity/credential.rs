use argentum_standard_business::data_type::id::Id;
use secrecy::SecretString;

// pub trait Credential: Send {}

#[derive(Clone)]
pub struct PasswordCredential<I>
where
    I: Id,
{
    pub user_id: I,
    pub password: SecretString,
}

// impl<I> Credential for PasswordCredential<I> where I: Id {}

impl<I> PasswordCredential<I>
where
    I: Id,
{
    pub fn new(user_id: I, password: SecretString) -> Self {
        Self { user_id, password }
    }
}

#[derive(Clone)]
pub struct EncodedPasswordCredential<I>
where
    I: Id,
{
    pub user_id: I,
    pub password: SecretString,
    pub salt: SecretString,
}

impl<I> EncodedPasswordCredential<I>
where
    I: Id,
{
    pub fn new(user_id: I, password: SecretString, salt: SecretString) -> Self {
        Self {
            user_id,
            password,
            salt,
        }
    }
}
