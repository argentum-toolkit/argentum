use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use argentum_standard_business::{data_type::id::UIntId, error::PoisonedLockError};

use crate::{
    entity::user::AuthenticatedUser,
    service::{MemberAuthenticatorError, MemberAuthenticatorTrait},
};

pub struct MemberAuthenticatorMock {
    users: RwLock<HashMap<String, AuthenticatedUser<UIntId>>>,
}

impl MemberAuthenticatorMock {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }

    #[cfg(test)]
    pub(crate) fn add(
        &self,
        token: &str,
        user: &AuthenticatedUser<UIntId>,
    ) -> Result<(), MemberAuthenticatorError> {
        // TODO: check if key exists

        match self
            .users
            .write()
            .map_err(|_| MemberAuthenticatorError::Other(Some(Arc::new(PoisonedLockError))))?
            .insert(token.into(), user.clone())
            .is_none()
        {
            true => Ok(()),
            false => Err(MemberAuthenticatorError::Other(None)),
        }
    }
}

#[async_trait]
impl MemberAuthenticatorTrait for MemberAuthenticatorMock {
    type Id = UIntId;

    async fn authenticate(
        &self,
        token: &str,
    ) -> Result<Option<AuthenticatedUser<Self::Id>>, MemberAuthenticatorError> {
        let guard = self
            .users
            .read()
            .map_err(|_| MemberAuthenticatorError::Other(Some(Arc::new(PoisonedLockError))))?;

        Ok(guard.get(token).cloned())
    }
}
