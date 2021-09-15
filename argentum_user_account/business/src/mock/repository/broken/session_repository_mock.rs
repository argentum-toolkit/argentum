use std::collections::HashMap;

use crate::entity::session::Session;
use crate::repository::session_repository::{SessionRepositoryError, SessionRepositoryTrait};
use argentum_standard_business::data_type::id::Id;
use std::sync::RwLock;

pub struct SessionRepositoryMockWithBrokenSave {
    sessions: RwLock<HashMap<Id, Session>>,
}

impl SessionRepositoryMockWithBrokenSave {
    pub fn new() -> SessionRepositoryMockWithBrokenSave {
        SessionRepositoryMockWithBrokenSave {
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for SessionRepositoryMockWithBrokenSave {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionRepositoryTrait for SessionRepositoryMockWithBrokenSave {
    fn find(&self, id: &Id) -> Option<Session> {
        self.sessions
            .read()
            .unwrap()
            .get(id)
            .map(|s| Session::new(s.id.clone(), s.user_id.clone(), s.token.clone()))
    }

    fn find_by_token(&self, token: String) -> Option<Session> {
        for (_, s) in self.sessions.read().unwrap().iter() {
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

    fn save(&self, _session: &Session) -> Result<(), SessionRepositoryError> {
        Err(SessionRepositoryError::Save)
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
