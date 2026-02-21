use crate::data_type::Name;
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use chrono::{DateTime, Utc};

pub trait UserTrait {
    type Id: Id;

    fn id(&self) -> Self::Id;
}

//============== AUTHENTICATED USER
#[derive(Clone)]
pub struct AuthenticatedUser<I>
where
    I: Id,
{
    pub id: I,
    pub created_at: DateTime<Utc>,
    pub name: Name,
    pub email: EmailAddress,
    //TODO: pub verified: bool,
}

impl<I> AuthenticatedUser<I>
where
    I: Id,
{
    pub fn new(id: I, name: Name, email: EmailAddress) -> Self {
        AuthenticatedUser {
            id,
            created_at: Utc::now(),
            name,
            email,
        }
    }
}

impl<I> UserTrait for AuthenticatedUser<I>
where
    I: Id,
{
    type Id = I;
    fn id(&self) -> Self::Id {
        self.id
    }
}

//============== ANONYMOUS USER

#[derive(Clone)]
pub struct AnonymousUser<I>
where
    I: Id,
{
    pub id: I,
    pub created_at: DateTime<Utc>,
}

impl<I> AnonymousUser<I>
where
    I: Id,
{
    pub fn new(id: I) -> Self {
        AnonymousUser {
            id,
            created_at: Utc::now(),
        }
    }
}

impl<I> UserTrait for AnonymousUser<I>
where
    I: Id,
{
    type Id = I;
    fn id(&self) -> Self::Id {
        self.id
    }
}

//============== ENUM USER allows operating with anonymous as with authenticated users
#[derive(Clone)]
pub enum User<I>
where
    I: Id,
{
    Anonymous(AnonymousUser<I>),         //Guest
    Authenticated(AuthenticatedUser<I>), //Member
}
