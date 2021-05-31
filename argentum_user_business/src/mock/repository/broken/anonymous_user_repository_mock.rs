use crate::entity::user::AnonymousUser;
use crate::repository::user_repository::{AnonymousUserRepositoryTrait, SavingUserError};
use argentum_standard_business::data_type::id::Id;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct AnonymousRepositoryMockWithBrokenSave {
    users: RefCell<HashMap<Id, AnonymousUser>>,
}

impl AnonymousRepositoryMockWithBrokenSave {
    pub fn new() -> AnonymousRepositoryMockWithBrokenSave {
        AnonymousRepositoryMockWithBrokenSave {
            users: RefCell::new(HashMap::new()),
        }
    }
}

impl Default for AnonymousRepositoryMockWithBrokenSave {
    fn default() -> Self {
        Self::new()
    }
}

impl AnonymousUserRepositoryTrait for AnonymousRepositoryMockWithBrokenSave {
    fn find(&self, id: &Id) -> Option<AnonymousUser> {
        self.users.borrow().get(id).map(|u| AnonymousUser {
            id: u.id.clone(),
            created_at: u.created_at,
        })
    }

    fn save(&self, _user: &AnonymousUser) -> Result<(), SavingUserError> {
        Err(SavingUserError::Anonymous)
    }
}
