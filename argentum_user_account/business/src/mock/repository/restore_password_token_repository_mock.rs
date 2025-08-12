use std::collections::HashMap;

use crate::entity::restore_password_token::RestorePasswordToken;
use crate::repository::restore_password_token_repository::{
    RestorePasswordTokenRepositoryError, RestorePasswordTokenRepositoryTrait,
};
use argentum_standard_business::data_type::id::Id;
use std::sync::RwLock;

pub struct RestorePasswordTokenRepositoryMock {
    tokens: RwLock<HashMap<Id, RestorePasswordToken>>,
}

impl RestorePasswordTokenRepositoryMock {
    pub fn new() -> RestorePasswordTokenRepositoryMock {
        RestorePasswordTokenRepositoryMock {
            tokens: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for RestorePasswordTokenRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl RestorePasswordTokenRepositoryTrait for RestorePasswordTokenRepositoryMock {
    fn find(
        &self,
        id: &Id,
    ) -> Result<Option<RestorePasswordToken>, RestorePasswordTokenRepositoryError> {
        let guard = self.tokens.read().map_err(|_| {
            RestorePasswordTokenRepositoryError::Other(Some("`RwLock` is poisoned".into()))
        })?;

        let result = guard
            .get(id)
            .map(|t| RestorePasswordToken::new(t.id.clone(), t.user_id.clone(), t.token.clone()));

        Ok(result)
    }

    fn find_by_token(
        &self,
        token: String,
    ) -> Result<Option<RestorePasswordToken>, RestorePasswordTokenRepositoryError> {
        let guard = self.tokens.read().map_err(|_| {
            RestorePasswordTokenRepositoryError::Other(Some("`RwLock` is poisoned".into()))
        })?;

        for (_, t) in guard.iter() {
            if t.token == token {
                return Ok(Some(RestorePasswordToken {
                    id: t.id.clone(),
                    user_id: t.user_id.clone(),
                    token: t.token.clone(),
                    created_at: t.created_at,
                }));
            }
        }

        Ok(None)
    }

    fn save(
        &self,
        token: &RestorePasswordToken,
    ) -> Result<(), RestorePasswordTokenRepositoryError> {
        // TODO: check if key exists

        let t = RestorePasswordToken {
            id: token.id.clone(),
            user_id: token.user_id.clone(),
            token: token.token.clone(),
            created_at: token.created_at,
        };

        match self
            .tokens
            .write()
            .map_err(|_| {
                RestorePasswordTokenRepositoryError::Other(Some("`RwLock` is poisoned".into()))
            })?
            .insert(token.id.clone(), t)
            .is_none()
        {
            true => Ok(()),
            false => Err(RestorePasswordTokenRepositoryError::Save(None)),
        }
    }

    fn delete_users_tokens(&self, user_id: &Id) -> Result<(), RestorePasswordTokenRepositoryError> {
        let mut id: Option<Id> = None;

        let guard = self.tokens.read().map_err(|_| {
            RestorePasswordTokenRepositoryError::Other(Some("`RwLock` is poisoned".into()))
        })?;

        for (k, t) in guard.iter() {
            if &t.user_id == user_id {
                id = Some(k.clone());

                break;
            }
        }

        drop(guard);

        if let Some(id) = id {
            self.tokens
                .write()
                .map_err(|_| {
                    RestorePasswordTokenRepositoryError::Other(Some("`RwLock` is poisoned".into()))
                })?
                .remove(&id);
        }

        Ok(())
    }
}
