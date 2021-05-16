use crate::entity::user::{AuthenticatedUser, UserTrait};
use crate::repository::user_repository::{AuthenticatedUserRepositoryTrait, SavingUserError};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::IdTrait;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct AuthenticatedUserRepositoryMock<'a> {
    users: RefCell<HashMap<Box<dyn IdTrait>, AuthenticatedUser>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> AuthenticatedUserRepositoryMock<'a> {
    pub fn new() -> AuthenticatedUserRepositoryMock<'a> {
        AuthenticatedUserRepositoryMock {
            users: RefCell::new(HashMap::new()),
            phantom: Default::default(),
        }
    }
}

impl<'a> AuthenticatedUserRepositoryTrait for AuthenticatedUserRepositoryMock<'a> {
    fn find(&self, id: &Box<dyn IdTrait>) -> Option<AuthenticatedUser> {
        match self.users.borrow().get(id) {
            None => None,
            Some(u) => Some(AuthenticatedUser::new(
                &u.id().clone(),
                u.name.clone(),
                u.email.clone(),
            )),
        }
    }

    fn find_by_email(&self, email: &EmailAddress) -> Option<AuthenticatedUser> {
        for (_, u) in self.users.borrow().iter() {
            if &u.email == email {
                return Some(AuthenticatedUser::new(
                    &u.id().clone(),
                    u.name.clone(),
                    u.email.clone(),
                ));
            }
        }

        None
    }

    fn save(&self, user: &AuthenticatedUser) -> Result<(), SavingUserError> {
        // TODO: check if key exists
        // if self.users. contains_key(user.get_id().clone()) {
        //     return Err("Already exists".parse().unwrap());
        // }

        let u = AuthenticatedUser {
            id: user.id().clone(),
            created_at: user.created_at,
            name: user.name.clone(),
            email: user.email.clone(),
        };

        match self
            .users
            .borrow_mut()
            .insert(user.id().clone(), u)
            .is_none()
        {
            true => Ok(()),
            false => Err(SavingUserError::Authenticated),
        }
    }
}
