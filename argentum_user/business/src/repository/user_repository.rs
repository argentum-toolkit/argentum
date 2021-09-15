use crate::entity::user::{AnonymousUser, AuthenticatedUser};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::Id;

pub trait AuthenticatedUserRepositoryTrait: Send + Sync {
    fn find(&self, id: &Id) -> Result<Option<AuthenticatedUser>, ExternalUserError>;
    fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<AuthenticatedUser>, ExternalUserError>;
    fn save(&self, user: &AuthenticatedUser) -> Result<(), ExternalUserError>;
}

pub trait AnonymousUserRepositoryTrait: Send + Sync {
    fn find(&self, id: &Id) -> Option<AnonymousUser>;
    fn save(&self, user: &AnonymousUser) -> Result<(), ExternalUserError>;
}

#[derive(thiserror::Error, Debug)]
pub enum ExternalUserError {
    #[error("Can't save an user")]
    Authenticated,

    #[error("Can't save an anonymous")]
    Anonymous,
}
