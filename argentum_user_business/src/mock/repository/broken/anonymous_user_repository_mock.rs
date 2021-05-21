use crate::entity::user::AnonymousUser;
use crate::repository::user_repository::{AnonymousUserRepositoryTrait, SavingUserError};
use argentum_standard_business::data_type::id::IdTrait;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct AnonymousRepositoryMockWithBrokenSave<'a> {
    users: RefCell<HashMap<Box<dyn IdTrait>, AnonymousUser>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> AnonymousRepositoryMockWithBrokenSave<'a> {
    pub fn new() -> AnonymousRepositoryMockWithBrokenSave<'a> {
        AnonymousRepositoryMockWithBrokenSave {
            users: RefCell::new(HashMap::new()),
            phantom: Default::default(),
        }
    }
}

impl<'a> Default for AnonymousRepositoryMockWithBrokenSave<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> AnonymousUserRepositoryTrait for AnonymousRepositoryMockWithBrokenSave<'a> {
    fn find(&self, id: &Box<dyn IdTrait>) -> Option<AnonymousUser> {
        self.users.borrow().get(id).map(|u| AnonymousUser {
            id: u.id.clone(),
            created_at: u.created_at,
        })
    }

    fn save(&self, _user: &AnonymousUser) -> Result<(), SavingUserError> {
        Err(SavingUserError::Anonymous)
    }
}
