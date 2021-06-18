use crate::entity::user::{AuthenticatedUser, UserTrait};
use crate::repository::user_repository::{AuthenticatedUserRepositoryTrait, SavingUserError};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct AuthenticatedUserRepositoryMockWihBrokenSave {
    users: RefCell<HashMap<Id, AuthenticatedUser>>,
}

impl AuthenticatedUserRepositoryMockWihBrokenSave {
    pub fn new() -> AuthenticatedUserRepositoryMockWihBrokenSave {
        AuthenticatedUserRepositoryMockWihBrokenSave {
            users: RefCell::new(HashMap::new()),
        }
    }
}

impl Default for AuthenticatedUserRepositoryMockWihBrokenSave {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthenticatedUserRepositoryTrait for AuthenticatedUserRepositoryMockWihBrokenSave {
    fn find(&self, id: &Id) -> Result<Option<AuthenticatedUser>, SavingUserError> {
        Ok(self
            .users
            .borrow()
            .get(id)
            .map(|u| AuthenticatedUser::new(&u.id().clone(), u.name.clone(), u.email.clone())))
    }

    fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<AuthenticatedUser>, SavingUserError> {
        for (_, u) in self.users.borrow().iter() {
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

    fn save(&self, _user: &AuthenticatedUser) -> Result<(), SavingUserError> {
        Err(SavingUserError::Authenticated)
    }
}
