use crate::entity::session::Session;
use argentum_standard_business::data_type::id::Id;
use std::error::Error;

pub trait SessionRepositoryTrait: Send + Sync {
    fn find_by_token(&self, token: String) -> Result<Option<Session>, SessionRepositoryError>;
    fn save(&self, session: &Session) -> Result<(), SessionRepositoryError>;
    fn delete_users_sessions(&self, user_id: &Id) -> Result<(), SessionRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum SessionRepositoryError {
    #[error("Can't save session")]
    Save(#[source] Option<Box<dyn Error>>),

    #[error("Can't delete session")]
    Delete(#[source] Option<Box<dyn Error>>),

    #[error("Session repository error")]
    Other(#[source] Option<Box<dyn Error>>),
}
