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
    fn find(&self, id: &Id) -> Result<Option<Session>, SessionRepositoryError> {
        let session = self
            .sessions
            .read()
            .unwrap()
            .get(id)
            .map(|s| Session::new(s.id.clone(), s.user_id.clone(), s.token.clone()));

        Ok(session)
    }

    fn find_by_token(&self, token: String) -> Result<Option<Session>, SessionRepositoryError> {
        for (_, s) in self.sessions.read().unwrap().iter() {
            if s.token == token {
                return Ok(Some(Session::new(
                    s.id.clone(),
                    s.user_id.clone(),
                    s.token.clone(),
                )));
            }
        }

        Ok(None)
    }

    fn save(&self, session: &Session) -> Result<(), SessionRepositoryError> {
        // TODO: check if key exists

        let s = Session::new(
            session.id.clone(),
            session.user_id.clone(),
            session.token.clone(),
        );

        match self
            .sessions
            .write()
            .unwrap()
            .insert(session.id.clone(), s)
            .is_none()
        {
            true => Ok(()),
            false => Err(SessionRepositoryError::Save(None)),
        }
    }

    fn delete_users_sessions(&self, user_id: &Id) -> Result<(), SessionRepositoryError> {
        let mut id: Option<Id> = None;

        for (k, s) in self.sessions.read().unwrap().iter() {
            if &s.user_id == user_id {
                id = Some(k.clone());

                break;
            }
        }

        if let Some(id) = id {
            self.sessions.write().unwrap().remove(&id);
        }

        Ok(())
    }
}
