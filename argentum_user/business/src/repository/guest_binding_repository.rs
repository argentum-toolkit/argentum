use crate::entity::guest_binding::GuestBinding;
use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait GuestBindingRepositoryTrait: Send + Sync {
    type Id: Id;
    async fn find_by_member_id(
        &self,
        member_id: Self::Id,
    ) -> Result<Option<GuestBinding<Self::Id>>, GuestBindingRepositoryError>;

    async fn save(
        &self,
        binding: &GuestBinding<Self::Id>,
    ) -> Result<(), GuestBindingRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum GuestBindingRepositoryError {
    #[error("Can't save guest binding")]
    Save(#[source] Option<Box<dyn Error>>),

    #[error("Can't find an guest binding")]
    Find(#[source] Option<Box<dyn Error>>),

    #[error("Guest binding repository error")]
    Other(#[source] Option<Box<dyn Error>>),
}
