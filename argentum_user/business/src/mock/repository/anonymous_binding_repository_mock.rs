use std::collections::HashMap;

use crate::entity::anonymous_binding::AnonymousBinding;
use crate::repository::anonymous_binding_repository::{
    AnonymousBindingRepositoryError, AnonymousBindingRepositoryTrait,
};
use argentum_standard_business::data_type::id::Id;
use std::sync::RwLock;

use async_trait::async_trait;

#[derive(Default)]
pub struct AnonymousBindingRepositoryMock<I>
where
    I: Id,
{
    sessions: RwLock<HashMap<I, AnonymousBinding<I>>>,
}

impl<I> AnonymousBindingRepositoryMock<I>
where
    I: Id,
{
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl<I> AnonymousBindingRepositoryTrait for AnonymousBindingRepositoryMock<I>
where
    I: Id,
{
    type Id = I;

    async fn find_by_user_id(
        &self,
        user_id: Self::Id,
    ) -> Result<Option<AnonymousBinding<Self::Id>>, AnonymousBindingRepositoryError> {
        let binding = self
            .sessions
            .read()
            .map_err(|_| {
                AnonymousBindingRepositoryError::Other(Some("`RwLock` is poisoned".into()))
            })?
            .get(&user_id)
            .map(|b| AnonymousBinding::<Self::Id>::new(b.user_id, b.anonymous_id));

        Ok(binding)
    }

    async fn save(
        &self,
        binding: &AnonymousBinding<Self::Id>,
    ) -> Result<(), AnonymousBindingRepositoryError> {
        // TODO: check if key exists; don't save if already bound

        let s = AnonymousBinding::<Self::Id>::new(binding.user_id, binding.anonymous_id);

        match self
            .sessions
            .write()
            .map_err(|_| {
                AnonymousBindingRepositoryError::Other(Some("`RwLock` is poisoned".into()))
            })?
            .insert(binding.user_id, s)
            .is_none()
        {
            true => Ok(()),
            false => Err(AnonymousBindingRepositoryError::Save(None)),
        }
    }
}
