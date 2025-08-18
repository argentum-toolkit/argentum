use std::collections::HashMap;

use crate::entity::session::Session;
use crate::repository::session_repository::{SessionRepositoryError, SessionRepositoryTrait};
use argentum_standard_business::data_type::id::Id;
use std::sync::RwLock;

pub struct SessionRepositoryMock {
    sessions: RwLock<HashMap<Id, Session>>,
}

impl SessionRepositoryMock {
    pub fn new() -> SessionRepositoryMock {
        SessionRepositoryMock {
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for SessionRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionRepositoryTrait for SessionRepositoryMock {
    fn find_by_token(&self, token: &str) -> Result<Option<Session>, SessionRepositoryError> {
        let guard = self
            .sessions
            .read()
            .map_err(|_| SessionRepositoryError::Other(Some("`RwLock` is poisoned".into())))?;

        for (_, s) in guard.iter() {
            if s.token == token {
                return Ok(Some(Session::new(
                    s.id.clone(),
                    s.user_id.clone(),
                    &s.token,
                )));
            }
        }

        Ok(None)
    }

    fn save(&self, session: &Session) -> Result<(), SessionRepositoryError> {
        // TODO: check if key exists

        let s = Session::new(session.id.clone(), session.user_id.clone(), &session.token);

        match self
            .sessions
            .write()
            .map_err(|_| SessionRepositoryError::Other(Some("`RwLock` is poisoned".into())))?
            .insert(session.id.clone(), s)
            .is_none()
        {
            true => Ok(()),
            false => Err(SessionRepositoryError::Save(None)),
        }
    }

    fn delete_users_sessions(&self, user_id: &Id) -> Result<(), SessionRepositoryError> {
        let mut id: Option<Id> = None;

        let guard = self
            .sessions
            .read()
            .map_err(|_| SessionRepositoryError::Other(Some("`RwLock` is poisoned".into())))?;

        for (k, s) in guard.iter() {
            if &s.user_id == user_id {
                id = Some(k.clone());

                break;
            }
        }

        if let Some(id) = id {
            self.sessions
                .write()
                .map_err(|_| SessionRepositoryError::Other(Some("`RwLock` is poisoned".into())))?
                .remove(&id);
        }

        Ok(())
    }
}
