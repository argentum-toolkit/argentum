use crate::entity::user::{AuthenticatedUser, UserTrait};
use crate::repository::user_repository::{AuthenticatedUserRepositoryTrait, SavingUserError};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::IdTrait;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct AuthenticatedUserRepositoryMockWihBrokenSave<'a> {
    users: RefCell<HashMap<Box<dyn IdTrait>, AuthenticatedUser>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> AuthenticatedUserRepositoryMockWihBrokenSave<'a> {
    pub fn new() -> AuthenticatedUserRepositoryMockWihBrokenSave<'a> {
        AuthenticatedUserRepositoryMockWihBrokenSave {
            users: RefCell::new(HashMap::new()),
            phantom: Default::default(),
        }
    }
}

impl<'a> Default for AuthenticatedUserRepositoryMockWihBrokenSave<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> AuthenticatedUserRepositoryTrait for AuthenticatedUserRepositoryMockWihBrokenSave<'a> {
    fn find(&self, id: &Box<dyn IdTrait>) -> Option<AuthenticatedUser> {
        self.users
            .borrow()
            .get(id)
            .map(|u| AuthenticatedUser::new(&u.id().clone(), u.name.clone(), u.email.clone()))
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

    fn save(&self, _user: &AuthenticatedUser) -> Result<(), SavingUserError> {
        Err(SavingUserError::Authenticated)
    }
}
