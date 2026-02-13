use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::service::{
    PasswordToTokenExchangeResult, PasswordToTokenExchangerError, PasswordToTokenExchangerTrait,
};

use argentum_standard_business::{data_type::id::UIntId, error::PoisonedLockError};
use argentum_user_business::entity::session::Session;
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};

use argentum_standard_business::data_type::EmailAddress;

pub struct PasswordToTokenExchangerMock {
    data: RwLock<HashMap<(String, String), PasswordToTokenExchangeResult<UIntId>>>,
}

impl PasswordToTokenExchangerMock {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn seed(
        &self,
        email: EmailAddress,
        password: SecretString,
        result: PasswordToTokenExchangeResult<UIntId>,
    ) -> Result<&Self, PasswordToTokenExchangerError> {
        self.data
            .write()
            .map_err(|_| PasswordToTokenExchangerError::Other(Some(Arc::new(PoisonedLockError))))?
            .insert((email.as_string(), password.expose_secret().into()), result);

        Ok(self)
    }
}

#[async_trait]
impl PasswordToTokenExchangerTrait for PasswordToTokenExchangerMock {
    type Id = UIntId;

    async fn exchange_password(
        &self,
        email: &EmailAddress,
        password: &SecretString,
    ) -> PasswordToTokenExchangeResult<Self::Id> {
        let result = self
            .data
            .read()
            .map_err(|_| PasswordToTokenExchangerError::Other(Some(Arc::new(PoisonedLockError))))?
            .get(&(email.as_string(), password.expose_secret().into()))
            .ok_or(PasswordToTokenExchangerError::Other(None))
            .map(|r| match r {
                Ok(s) => Ok(Session::new(s.id, s.user_id, &s.token)),
                Err(e) => Err(e.clone()),
            })?;

        result
    }
}
