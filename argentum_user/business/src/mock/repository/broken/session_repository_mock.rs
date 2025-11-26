use std::collections::HashMap;

use crate::entity::session::GuestSession;
use crate::repository::session_repository::{SessionRepositoryError, SessionRepositoryTrait};
use argentum_standard_business::data_type::id::UIntId;
use argentum_standard_business::error::PoisonedLockError;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

pub struct SessionRepositoryMockWithBrokenSave {
    sessions: RwLock<HashMap<UIntId, GuestSession<UIntId>>>,
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

#[async_trait]
impl SessionRepositoryTrait for SessionRepositoryMockWithBrokenSave {
    type Id = UIntId;

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<GuestSession<Self::Id>>, SessionRepositoryError> {
        let guard = self
            .sessions
            .read()
            .map_err(|_| SessionRepositoryError::Other(Some(Arc::new(PoisonedLockError))))?;

        for (_, s) in guard.iter() {
            if s.token == token {
                return Ok(Some(GuestSession::<Self::Id>::new(
                    s.id, s.user_id, &s.token,
                )));
            }
        }

        Ok(None)
    }

    async fn save(&self, _session: &GuestSession<Self::Id>) -> Result<(), SessionRepositoryError> {
        Err(SessionRepositoryError::Save(None))
    }

    async fn delete_users_sessions(&self, user_id: Self::Id) -> Result<(), SessionRepositoryError> {
        let mut id: Option<Self::Id> = None;

        let read_guard = self
            .sessions
            .read()
            .map_err(|_| SessionRepositoryError::Other(Some(Arc::new(PoisonedLockError))))?;

        for (k, s) in read_guard.iter() {
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
