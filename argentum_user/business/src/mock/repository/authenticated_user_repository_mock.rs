use crate::entity::user::{AuthenticatedUser, UserTrait};
use crate::repository::user_repository::{AuthenticatedUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct AuthenticatedUserRepositoryMock {
    users: RwLock<HashMap<Id, AuthenticatedUser>>,
}

impl AuthenticatedUserRepositoryMock {
    pub fn new() -> AuthenticatedUserRepositoryMock {
        AuthenticatedUserRepositoryMock {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for AuthenticatedUserRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthenticatedUserRepositoryTrait for AuthenticatedUserRepositoryMock {
    fn find(&self, id: &Id) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        Ok(self
            .users
            .read()
            .map_err(|_| ExternalUserError::Authenticated(Some("`RwLock` is poisoned".into())))?
            .get(id)
            .map(|u| AuthenticatedUser::new(&u.id().clone(), u.name.clone(), u.email.clone())))
    }

    fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        let guard = self
            .users
            .read()
            .map_err(|_| ExternalUserError::Authenticated(Some("`RwLock` is poisoned".into())))?;

        for (_, u) in guard.iter() {
            if &u.email == email {
                return Ok(Some(AuthenticatedUser::new(
                    &u.id().clone(),
                    u.name.clone(),
                    u.email.clone(),
                )));
            }
        }

        Ok(None)
    }

    fn save(&self, user: &AuthenticatedUser) -> Result<(), ExternalUserError> {
        // TODO: check if key exists. Eg:
        // if self.users. contains_key(user.get_id().clone()) {
        //     return Err("Already exists");
        // }

        let u = AuthenticatedUser {
            id: user.id().clone(),
            created_at: user.created_at,
            name: user.name.clone(),
            email: user.email.clone(),
        };

        match self
            .users
            .write()
            .map_err(|_| ExternalUserError::Authenticated(Some("`RwLock` is poisoned".into())))?
            .insert(user.id().clone(), u)
            .is_none()
        {
            true => Ok(()),
            false => Err(ExternalUserError::Authenticated(None)),
        }
    }
}
