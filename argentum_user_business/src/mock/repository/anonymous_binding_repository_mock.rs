use std::cell::RefCell;
use std::collections::HashMap;

use crate::entity::anonymous_binding::AnonymousBinding;
use crate::repository::anonymous_binding_repository::{
    AnonymousBindingRepositoryError, AnonymousBindingRepositoryTrait,
};
use argentum_standard_business::data_type::id::Id;

pub struct AnonymousBindingRepositoryMock {
    sessions: RefCell<HashMap<Id, AnonymousBinding>>,
}

impl AnonymousBindingRepositoryMock {
    pub fn new() -> AnonymousBindingRepositoryMock {
        AnonymousBindingRepositoryMock {
            sessions: RefCell::new(HashMap::new()),
        }
    }
}

impl Default for AnonymousBindingRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl AnonymousBindingRepositoryTrait for AnonymousBindingRepositoryMock {
    fn find_by_user_id(&self, user_id: &Id) -> Option<AnonymousBinding> {
        self.sessions
            .borrow()
            .get(user_id)
            .map(|b| AnonymousBinding::new(b.user_id.clone(), b.anonymous_id.clone()))
    }

    fn save(&self, binding: &AnonymousBinding) -> Result<(), AnonymousBindingRepositoryError> {
        // TODO: check if key exists; don't save if already bound

        let s = AnonymousBinding::new(binding.user_id.clone(), binding.anonymous_id.clone());

        match self
            .sessions
            .borrow_mut()
            .insert(binding.user_id.clone(), s)
            .is_none()
        {
            true => Ok(()),
            false => Err(AnonymousBindingRepositoryError::Save),
        }
    }
}
