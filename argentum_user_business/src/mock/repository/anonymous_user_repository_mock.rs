use crate::entity::user::{AnonymousUser, UserTrait};
use crate::repository::user_repository::{AnonymousUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::id::Id;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct AnonymousUserRepositoryMock {
    users: RefCell<HashMap<Id, AnonymousUser>>,
}

impl AnonymousUserRepositoryMock {
    pub fn new() -> AnonymousUserRepositoryMock {
        AnonymousUserRepositoryMock {
            users: RefCell::new(HashMap::new()),
        }
    }
}

impl Default for AnonymousUserRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl AnonymousUserRepositoryTrait for AnonymousUserRepositoryMock {
    fn find(&self, id: &Id) -> Option<AnonymousUser> {
        self.users.borrow().get(id).map(|u| AnonymousUser {
            id: u.id.clone(),
            created_at: u.created_at,
        })
    }

    fn save(&self, user: &AnonymousUser) -> Result<(), ExternalUserError> {
        // TODO: check if key exists

        let u = AnonymousUser {
            id: user.id().clone(),
            created_at: user.created_at,
        };

        match self
            .users
            .borrow_mut()
            .insert(user.id().clone(), u)
            .is_none()
        {
            true => Ok(()),
            false => Err(ExternalUserError::Anonymous),
        }
    }
}
