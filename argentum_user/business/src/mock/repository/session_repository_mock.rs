use std::collections::HashMap;

use crate::entity::session::Session;
use crate::repository::session_repository::{SessionRepositoryError, SessionRepositoryTrait};
use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;

use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;

#[derive(Default)]
pub struct SessionRepositoryMock<I>
where
    I: Id,
{
    sessions: RwLock<HashMap<I, Session<I>>>,
}

impl<I> SessionRepositoryMock<I>
where
    I: Id,
{
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

#[derive(Debug)]
struct PoisonedLockError;

impl std::fmt::Display for PoisonedLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "`RwLock` is poisoned")
    }
}

impl Error for PoisonedLockError {}

#[async_trait]
impl<I> SessionRepositoryTrait for SessionRepositoryMock<I>
where
    I: Id,
{
    type Id = I;

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<Session<Self::Id>>, SessionRepositoryError> {
        let guard = self
            .sessions
            .read()
            .map_err(|_| SessionRepositoryError::Other(Some(Arc::new(PoisonedLockError))))?;

        for (_, s) in guard.iter() {
            if s.token == token {
                return Ok(Some(Session::new(s.id, s.user_id, &s.token)));
            }
        }

        Ok(None)
    }

    async fn save(&self, session: &Session<Self::Id>) -> Result<(), SessionRepositoryError> {
        // TODO: check if key exists

        let s = Session::<Self::Id>::new(session.id, session.user_id, &session.token);

        match self
            .sessions
            .write()
            .map_err(|_| SessionRepositoryError::Other(Some(Arc::new(PoisonedLockError))))?
            .insert(session.id, s)
            .is_none()
        {
            true => Ok(()),
            false => Err(SessionRepositoryError::Save(None)),
        }
    }

    async fn delete_users_sessions(&self, user_id: Self::Id) -> Result<(), SessionRepositoryError> {
        let mut id: Option<Self::Id> = None;

        let guard = self
            .sessions
            .read()
            .map_err(|_| SessionRepositoryError::Other(Some(Arc::new(PoisonedLockError))))?;

        for (k, s) in guard.iter() {
            if s.user_id == user_id {
                id = Some(*k);

                break;
            }
        }

        if let Some(id) = id {
            self.sessions
                .write()
                .map_err(|_| SessionRepositoryError::Other(Some(Arc::new(PoisonedLockError))))?
                .remove(&id);
        }

        Ok(())
    }
}
