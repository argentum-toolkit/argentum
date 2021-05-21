use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::entity::anonymous_binding::AnonymousBinding;
use crate::repository::anonymous_binding_repository::{
    AnonymousBindingRepositoryError, AnonymousBindingRepositoryTrait,
};
use argentum_standard_business::data_type::id::IdTrait;

pub struct AnonymousBindingRepositoryMock<'a> {
    sessions: RefCell<HashMap<Box<dyn IdTrait>, AnonymousBinding>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> AnonymousBindingRepositoryMock<'a> {
    pub fn new() -> AnonymousBindingRepositoryMock<'a> {
        AnonymousBindingRepositoryMock {
            sessions: RefCell::new(HashMap::new()),
            phantom: Default::default(),
        }
    }
}

impl<'a> Default for AnonymousBindingRepositoryMock<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> AnonymousBindingRepositoryTrait for AnonymousBindingRepositoryMock<'a> {
    fn find_by_user_id(&self, user_id: &Box<dyn IdTrait>) -> Option<AnonymousBinding> {
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
