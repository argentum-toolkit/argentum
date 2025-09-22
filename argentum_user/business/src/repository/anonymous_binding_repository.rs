use crate::entity::anonymous_binding::AnonymousBinding;
use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait AnonymousBindingRepositoryTrait: Send + Sync {
    type Id: Id;
    async fn find_by_user_id(
        &self,
        user_id: Self::Id,
    ) -> Result<Option<AnonymousBinding<Self::Id>>, AnonymousBindingRepositoryError>;

    async fn save(
        &self,
        binding: &AnonymousBinding<Self::Id>,
    ) -> Result<(), AnonymousBindingRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum AnonymousBindingRepositoryError {
    #[error("Can't save anonymous binding")]
    Save(#[source] Option<Box<dyn Error>>),

    #[error("Can't find an anonymous binding")]
    Find(#[source] Option<Box<dyn Error>>),

    #[error("Anonymous binding repository error")]
    Other(#[source] Option<Box<dyn Error>>),
}
