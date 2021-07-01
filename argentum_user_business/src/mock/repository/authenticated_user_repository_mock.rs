use crate::entity::user::{AuthenticatedUser, UserTrait};
use crate::repository::user_repository::{AuthenticatedUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct AuthenticatedUserRepositoryMock {
    users: RefCell<HashMap<Id, AuthenticatedUser>>,
}

impl AuthenticatedUserRepositoryMock {
    pub fn new() -> AuthenticatedUserRepositoryMock {
        AuthenticatedUserRepositoryMock {
            users: RefCell::new(HashMap::new()),
        }
    }
}

impl Default for AuthenticatedUserRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthenticatedUserRepositoryTrait for AuthenticatedUserRepositoryMock {
    fn find(&self, id: &Id) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        Ok(self
            .users
            .borrow()
            .get(id)
            .map(|u| AuthenticatedUser::new(&u.id().clone(), u.name.clone(), u.email.clone())))
    }

    fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
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

    fn save(&self, user: &AuthenticatedUser) -> Result<(), ExternalUserError> {
        // TODO: check if key exists
        // if self.users. contains_key(user.get_id().clone()) {
        //     return Err("Already exists".parse().unwrap());
        // }

        let u = AuthenticatedUser {
            id: user.id().clone(),
            created_at: user.created_at,
            name: user.name.clone(),
            email: user.email.clone(),
        };

        match self
            .users
            .borrow_mut()
            .insert(user.id().clone(), u)
            .is_none()
        {
            true => Ok(()),
            false => Err(ExternalUserError::Authenticated),
        }
    }
}
