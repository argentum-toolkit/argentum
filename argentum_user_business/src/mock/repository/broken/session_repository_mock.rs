use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::entity::session::Session;
use crate::repository::session_repository::{SessionRepositoryError, SessionRepositoryTrait};
use argentum_standard_business::data_type::id::IdTrait;

pub struct SessionRepositoryMockWithBrokenSave<'a> {
    sessions: RefCell<HashMap<Box<dyn IdTrait>, Session>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> SessionRepositoryMockWithBrokenSave<'a> {
    pub fn new() -> SessionRepositoryMockWithBrokenSave<'a> {
        SessionRepositoryMockWithBrokenSave {
            sessions: RefCell::new(HashMap::new()),
            phantom: Default::default(),
        }
    }
}

impl<'a> SessionRepositoryTrait for SessionRepositoryMockWithBrokenSave<'a> {
    fn find(&self, id: &Box<dyn IdTrait>) -> Option<Session> {
        match self.sessions.borrow().get(id) {
            None => None,
            Some(s) => Some(Session::new(
                s.id.clone(),
                s.user_id.clone(),
                s.token.clone(),
            )),
        }
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

    fn save(&self, _session: &Session) -> Result<(), SessionRepositoryError> {
        Err(SessionRepositoryError::Save)
    }

    fn delete_users_sessions(
        &self,
        user_id: &Box<dyn IdTrait>,
    ) -> Result<(), SessionRepositoryError> {
        for (k, s) in self.sessions.borrow().iter() {
            if &s.user_id == user_id {
                self.sessions.borrow_mut().remove(k);
            }
        }

        Ok(())
    }
}
