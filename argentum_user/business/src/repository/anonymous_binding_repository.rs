use crate::entity::anonymous_binding::AnonymousBinding;
use argentum_standard_business::data_type::id::Id;

pub trait AnonymousBindingRepositoryTrait: Send + Sync {
    fn find_by_user_id(&self, id: &Id) -> Option<AnonymousBinding>;
    fn save(&self, binding: &AnonymousBinding) -> Result<(), AnonymousBindingRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum AnonymousBindingRepositoryError {
    #[error("Can't save anonymous binding")]
    Save,
}
