use std::collections::HashMap;

use crate::entity::restore_password_token::RestorePasswordToken;
use crate::repository::restore_password_token_repository::{
    RestorePasswordTokenRepositoryError, RestorePasswordTokenRepositoryTrait,
};
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::error::PoisonedLockError;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

#[derive(Default)]
pub struct RestorePasswordTokenRepositoryMock<I>
where
    I: Id,
{
    tokens: RwLock<HashMap<I, RestorePasswordToken<I>>>,
}

impl<I> RestorePasswordTokenRepositoryMock<I>
where
    I: Id,
{
    pub fn new() -> Self {
        Self {
            tokens: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl<I> RestorePasswordTokenRepositoryTrait for RestorePasswordTokenRepositoryMock<I>
where
    I: Id,
{
    type Id = I;

    async fn find(
        &self,
        id: Self::Id,
    ) -> Result<Option<RestorePasswordToken<Self::Id>>, RestorePasswordTokenRepositoryError> {
        let guard = self.tokens.read().map_err(|_| {
            RestorePasswordTokenRepositoryError::Other(Some(Arc::new(PoisonedLockError)))
        })?;

        let result = guard
            .get(&id)
            .map(|t| RestorePasswordToken::new(t.id, t.user_id, &t.token));

        Ok(result)
    }

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<RestorePasswordToken<Self::Id>>, RestorePasswordTokenRepositoryError> {
        let guard = self.tokens.read().map_err(|_| {
            RestorePasswordTokenRepositoryError::Other(Some(Arc::new(PoisonedLockError)))
        })?;

        for (_, t) in guard.iter() {
            if t.token == token {
                return Ok(Some(RestorePasswordToken {
                    id: t.id,
                    user_id: t.user_id,
                    token: t.token.clone(),
                    created_at: t.created_at,
                }));
            }
        }

        Ok(None)
    }

    async fn save(
        &self,
        token: &RestorePasswordToken<Self::Id>,
    ) -> Result<(), RestorePasswordTokenRepositoryError> {
        // TODO: check if key exists

        let t = RestorePasswordToken {
            id: token.id,
            user_id: token.user_id,
            token: token.token.clone(),
            created_at: token.created_at,
        };

        match self
            .tokens
            .write()
            .map_err(|_| {
                RestorePasswordTokenRepositoryError::Other(Some(Arc::new(PoisonedLockError)))
            })?
            .insert(token.id, t)
            .is_none()
        {
            true => Ok(()),
            false => Err(RestorePasswordTokenRepositoryError::Save(None)),
        }
    }

    async fn delete_users_tokens(
        &self,
        user_id: Self::Id,
    ) -> Result<(), RestorePasswordTokenRepositoryError> {
        let mut id: Option<Self::Id> = None;

        let guard = self.tokens.read().map_err(|_| {
            RestorePasswordTokenRepositoryError::Other(Some(Arc::new(PoisonedLockError)))
        })?;

        for (k, t) in guard.iter() {
            if t.user_id == user_id {
                id = Some(*k);

                break;
            }
        }

        drop(guard);

        if let Some(id) = id {
            self.tokens
                .write()
                .map_err(|_| {
                    RestorePasswordTokenRepositoryError::Other(Some(Arc::new(PoisonedLockError)))
                })?
                .remove(&id);
        }

        Ok(())
    }
}
