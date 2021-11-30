use std::collections::HashMap;

use crate::entity::anonymous_binding::AnonymousBinding;
use crate::repository::anonymous_binding_repository::{
    AnonymousBindingRepositoryError, AnonymousBindingRepositoryTrait,
};
use argentum_standard_business::data_type::id::Id;
use std::sync::RwLock;

pub struct AnonymousBindingRepositoryMock {
    sessions: RwLock<HashMap<Id, AnonymousBinding>>,
}

impl AnonymousBindingRepositoryMock {
    pub fn new() -> AnonymousBindingRepositoryMock {
        AnonymousBindingRepositoryMock {
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for AnonymousBindingRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl AnonymousBindingRepositoryTrait for AnonymousBindingRepositoryMock {
    fn find_by_user_id(
        &self,
        user_id: &Id,
    ) -> Result<Option<AnonymousBinding>, AnonymousBindingRepositoryError> {
        let binding = self
            .sessions
            .read()
            .unwrap()
            .get(user_id)
            .map(|b| AnonymousBinding::new(b.user_id.clone(), b.anonymous_id.clone()));

        Ok(binding)
    }

    fn save(&self, binding: &AnonymousBinding) -> Result<(), AnonymousBindingRepositoryError> {
        // TODO: check if key exists; don't save if already bound

        let s = AnonymousBinding::new(binding.user_id.clone(), binding.anonymous_id.clone());

        match self
            .sessions
            .write()
            .unwrap()
            .insert(binding.user_id.clone(), s)
            .is_none()
        {
            true => Ok(()),
            false => Err(AnonymousBindingRepositoryError::Save(None)),
        }
    }
}
