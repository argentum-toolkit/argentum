use crate::entity::user::{AnonymousUser, AuthenticatedUser};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::Id;

// pub trait UserRepositoryTrait<I: IdTrait, U: UserTrait<I>> {
//     // fn find(&self, id: I) -> Result<U, Error>;
//     fn save(&self, user: U) -> Result<U, Error>;
// }

pub trait AuthenticatedUserRepositoryTrait {
    fn find(&self, id: &Id) -> Option<AuthenticatedUser>;
    fn find_by_email(&self, email: &EmailAddress) -> Option<AuthenticatedUser>;
    fn save(&self, user: &AuthenticatedUser) -> Result<(), SavingUserError>;
}

pub trait AnonymousUserRepositoryTrait {
    fn find(&self, id: &Id) -> Option<AnonymousUser>;
    fn save(&self, user: &AnonymousUser) -> Result<(), SavingUserError>;
}

#[derive(thiserror::Error, Debug)]
pub enum SavingUserError {
    #[error("Can't save an user")]
    Authenticated,

    #[error("Can't save an anonymous")]
    Anonymous,
}
