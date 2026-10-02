use std::collections::HashMap;

use crate::entity::EmailConfirmationToken;
use crate::repository::email_confirmation_token_repository::{
    EmailConfirmationTokenRepositoryError, EmailConfirmationTokenRepositoryTrait,
};
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::error::PoisonedLockError;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

#[derive(Default)]
pub struct EmailConfirmationTokenRepositoryMock<I>
where
    I: Id,
{
    tokens: RwLock<HashMap<I, EmailConfirmationToken<I>>>,
}

impl<I> EmailConfirmationTokenRepositoryMock<I>
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
impl<I> EmailConfirmationTokenRepositoryTrait for EmailConfirmationTokenRepositoryMock<I>
where
    I: Id,
{
    type Id = I;

    async fn find(
        &self,
        id: Self::Id,
    ) -> Result<Option<EmailConfirmationToken<Self::Id>>, EmailConfirmationTokenRepositoryError>
    {
        let guard = self.tokens.read().map_err(|_| {
            EmailConfirmationTokenRepositoryError::Other(Some(Arc::new(PoisonedLockError)))
        })?;

        let result = guard
            .get(&id)
            .map(|t| EmailConfirmationToken::new(t.id, t.user_id, &t.token));

        Ok(result)
    }

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<EmailConfirmationToken<Self::Id>>, EmailConfirmationTokenRepositoryError>
    {
        let guard = self.tokens.read().map_err(|_| {
            EmailConfirmationTokenRepositoryError::Other(Some(Arc::new(PoisonedLockError)))
        })?;

        for t in guard.values() {
            if t.token == token {
                return Ok(Some(EmailConfirmationToken {
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
        token: &EmailConfirmationToken<Self::Id>,
    ) -> Result<(), EmailConfirmationTokenRepositoryError> {
        // TODO: check if key exists

        let t = EmailConfirmationToken {
            id: token.id,
            user_id: token.user_id,
            token: token.token.clone(),
            created_at: token.created_at,
        };

        match self
            .tokens
            .write()
            .map_err(|_| {
                EmailConfirmationTokenRepositoryError::Other(Some(Arc::new(PoisonedLockError)))
            })?
            .insert(token.id, t)
            .is_none()
        {
            true => Ok(()),
            false => Err(EmailConfirmationTokenRepositoryError::Save(None)),
        }
    }

    async fn delete_users_tokens(
        &self,
        user_id: Self::Id,
    ) -> Result<(), EmailConfirmationTokenRepositoryError> {
        let mut id: Option<Self::Id> = None;

        let guard = self.tokens.read().map_err(|_| {
            EmailConfirmationTokenRepositoryError::Other(Some(Arc::new(PoisonedLockError)))
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
                    EmailConfirmationTokenRepositoryError::Other(Some(Arc::new(PoisonedLockError)))
                })?
                .remove(&id);
        }

        Ok(())
    }
}
