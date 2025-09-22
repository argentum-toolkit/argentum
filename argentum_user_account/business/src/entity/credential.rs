use argentum_standard_business::data_type::id::Id;

pub trait Credential: Send {}

#[derive(Clone)]
pub struct PasswordCredential<I>
where
    I: Id,
{
    pub user_id: I,
    pub password: String,
    pub salt: String,
}

impl<I> Credential for PasswordCredential<I> where I: Id {}

impl<I> PasswordCredential<I>
where
    I: Id,
{
    pub fn new(user_id: I, password: &str, salt: &str) -> Self {
        PasswordCredential {
            user_id,
            password: password.into(),
            salt: salt.into(),
        }
    }
}
