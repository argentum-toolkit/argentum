use crate::entity::anonymous_binding::AnonymousBinding;
use argentum_standard_business::data_type::id::Id;
use std::error::Error;

pub trait AnonymousBindingRepositoryTrait: Send + Sync {
    fn find_by_user_id(
        &self,
        user_id: &Id,
    ) -> Result<Option<AnonymousBinding>, AnonymousBindingRepositoryError>;

    fn save(&self, binding: &AnonymousBinding) -> Result<(), AnonymousBindingRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum AnonymousBindingRepositoryError {
    #[error("Can't save anonymous binding")]
    Save(#[source] Option<Box<dyn Error>>),

    #[error("Can't find an anonymous binding")]
    Find(#[source] Option<Box<dyn Error>>),
}
