use crate::entity::user::{AnonymousUser, UserTrait};
use crate::repository::user_repository::{AnonymousUserRepositoryTrait, SavingUserError};
use argentum_standard_business::data_type::id::IdTrait;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct AnonymousUserRepositoryMock<'a> {
    users: RefCell<HashMap<Box<dyn IdTrait>, AnonymousUser>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> AnonymousUserRepositoryMock<'a> {
    pub fn new() -> AnonymousUserRepositoryMock<'a> {
        AnonymousUserRepositoryMock {
            users: RefCell::new(HashMap::new()),
            phantom: Default::default(),
        }
    }
}

impl<'a> Default for AnonymousUserRepositoryMock<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> AnonymousUserRepositoryTrait for AnonymousUserRepositoryMock<'a> {
    fn find(&self, id: &Box<dyn IdTrait>) -> Option<AnonymousUser> {
        self.users.borrow().get(id).map(|u| AnonymousUser {
            id: u.id.clone(),
            created_at: u.created_at,
        })
    }

    fn save(&self, user: &AnonymousUser) -> Result<(), SavingUserError> {
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
            false => Err(SavingUserError::Anonymous),
        }
    }
}
