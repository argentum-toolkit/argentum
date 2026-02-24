use std::collections::HashMap;

use crate::entity::guest_binding::GuestBinding;
use crate::repository::guest_binding_repository::{
    GuestBindingRepositoryError, GuestBindingRepositoryTrait,
};
use argentum_standard_business::data_type::id::Id;
use std::sync::RwLock;

use async_trait::async_trait;

#[derive(Default)]
pub struct GuestBindingRepositoryMock<I>
where
    I: Id,
{
    sessions: RwLock<HashMap<I, GuestBinding<I>>>,
}

impl<I> GuestBindingRepositoryMock<I>
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
impl<I> GuestBindingRepositoryTrait for GuestBindingRepositoryMock<I>
where
    I: Id,
{
    type Id = I;

    async fn find_by_member_id(
        &self,
        user_id: Self::Id,
    ) -> Result<Option<GuestBinding<Self::Id>>, GuestBindingRepositoryError> {
        let binding = self
            .sessions
            .read()
            .map_err(|_| GuestBindingRepositoryError::Other(Some("`RwLock` is poisoned".into())))?
            .get(&user_id)
            .map(|b| GuestBinding::<Self::Id>::new(b.member_id, b.guest_id));

        Ok(binding)
    }

    async fn save(
        &self,
        binding: &GuestBinding<Self::Id>,
    ) -> Result<(), GuestBindingRepositoryError> {
        // TODO: check if key exists; don't save if already bound

        let s = GuestBinding::<Self::Id>::new(binding.member_id, binding.guest_id);

        match self
            .sessions
            .write()
            .map_err(|_| GuestBindingRepositoryError::Other(Some("`RwLock` is poisoned".into())))?
            .insert(binding.member_id, s)
            .is_none()
        {
            true => Ok(()),
            false => Err(GuestBindingRepositoryError::Save(None)),
        }
    }
}
