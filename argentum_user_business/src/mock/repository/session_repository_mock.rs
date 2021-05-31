use std::cell::RefCell;
use std::collections::HashMap;

use crate::entity::session::Session;
use crate::repository::session_repository::{SessionRepositoryError, SessionRepositoryTrait};
use argentum_standard_business::data_type::id::Id;

pub struct SessionRepositoryMock {
    sessions: RefCell<HashMap<Id, Session>>,
}

impl SessionRepositoryMock {
    pub fn new() -> SessionRepositoryMock {
        SessionRepositoryMock {
            sessions: RefCell::new(HashMap::new()),
        }
    }
}

impl Default for SessionRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionRepositoryTrait for SessionRepositoryMock {
    fn find(&self, id: &Id) -> Option<Session> {
        self.sessions
            .borrow()
            .get(id)
            .map(|s| Session::new(s.id.clone(), s.user_id.clone(), s.token.clone()))
    }

    fn find_by_token(&self, token: String) -> Option<Session> {
        for (_, s) in self.sessions.borrow().iter() {
            if s.token == token {
                return Some(Session::new(
                    s.id.clone(),
                    s.user_id.clone(),
                    s.token.clone(),
                ));
            }
        }

        None
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
            .borrow_mut()
            .insert(session.id.clone(), s)
            .is_none()
        {
            true => Ok(()),
            false => Err(SessionRepositoryError::Save),
        }
    }

    fn delete_users_sessions(&self, user_id: &Id) -> Result<(), SessionRepositoryError> {
        for (k, s) in self.sessions.borrow().iter() {
            if &s.user_id == user_id {
                self.sessions.borrow_mut().remove(k);
            }
        }

        Ok(())
    }
}
