use crate::entity::user::{AuthenticatedUser, UserTrait};
use crate::repository::user_repository::{AuthenticatedUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct AuthenticatedUserRepositoryMockWihBrokenSave {
    users: RwLock<HashMap<Id, AuthenticatedUser>>,
}

impl AuthenticatedUserRepositoryMockWihBrokenSave {
    pub fn new() -> AuthenticatedUserRepositoryMockWihBrokenSave {
        AuthenticatedUserRepositoryMockWihBrokenSave {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for AuthenticatedUserRepositoryMockWihBrokenSave {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthenticatedUserRepositoryTrait for AuthenticatedUserRepositoryMockWihBrokenSave {
    fn find(&self, id: &Id) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        Ok(self
            .users
            .read()
            .unwrap()
            .get(id)
            .map(|u| AuthenticatedUser::new(&u.id().clone(), u.name.clone(), u.email.clone())))
    }

    fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        for (_, u) in self.users.read().unwrap().iter() {
            if &u.email == email {
                return Ok(Some(AuthenticatedUser::new(
                    &u.id().clone(),
                    u.name.clone(),
                    u.email.clone(),
                )));
            }
        }

        Ok(None)
    }

    fn save(&self, _user: &AuthenticatedUser) -> Result<(), ExternalUserError> {
        Err(ExternalUserError::Authenticated(None))
    }
}
