use crate::entity::user::AnonymousUser;
use crate::repository::user_repository::{AnonymousUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::id::Id;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct AnonymousRepositoryMockWithBrokenSave {
    users: RwLock<HashMap<Id, AnonymousUser>>,
}

impl AnonymousRepositoryMockWithBrokenSave {
    pub fn new() -> AnonymousRepositoryMockWithBrokenSave {
        AnonymousRepositoryMockWithBrokenSave {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for AnonymousRepositoryMockWithBrokenSave {
    fn default() -> Self {
        Self::new()
    }
}

impl AnonymousUserRepositoryTrait for AnonymousRepositoryMockWithBrokenSave {
    fn find(&self, id: &Id) -> Result<Option<AnonymousUser>, ExternalUserError> {
        Ok(self.users.read().unwrap().get(id).map(|u| AnonymousUser {
            id: u.id.clone(),
            created_at: u.created_at,
        }))
    }

    fn save(&self, _user: &AnonymousUser) -> Result<(), ExternalUserError> {
        Err(ExternalUserError::Anonymous(None))
    }
}
