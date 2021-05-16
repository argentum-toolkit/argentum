use crate::entity::anonymous_binding::AnonymousBinding;
use argentum_standard_business::data_type::id::IdTrait;

pub trait AnonymousBindingRepositoryTrait {
    fn find_by_user_id(&self, id: &Box<dyn IdTrait>) -> Option<AnonymousBinding>;
    fn save(&self, binding: &AnonymousBinding) -> Result<(), AnonymousBindingRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum AnonymousBindingRepositoryError {
    #[error("Can't save anonymous binding")]
    Save,
}
