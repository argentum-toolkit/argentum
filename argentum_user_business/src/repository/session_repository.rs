use crate::entity::session::Session;
use argentum_standard_business::data_type::id::IdTrait;

pub trait SessionRepositoryTrait {
    fn find(&self, id: &Box<dyn IdTrait>) -> Option<Session>;
    fn find_by_token(&self, token: String) -> Option<Session>;
    fn save(&self, session: &Session) -> Result<(), SessionRepositoryError>;
    fn delete_users_sessions(
        &self,
        user_id: &Box<dyn IdTrait>,
    ) -> Result<(), SessionRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum SessionRepositoryError {
    #[error("Can't save session")]
    Save,

    #[error("Can't delete session")]
    Delete,
}
