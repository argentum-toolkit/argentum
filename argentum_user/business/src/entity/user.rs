use crate::data_type::Name;
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use chrono::{DateTime, Utc};

pub trait UserTrait {
    type Id: Id;

    fn id(&self) -> Self::Id;
}

//============== MEMBER USER
#[derive(Clone)]
pub struct Member<I>
where
    I: Id,
{
    pub id: I,
    pub created_at: DateTime<Utc>,
    pub name: Name,
    pub email: EmailAddress,
    //TODO: pub verified: bool,
}

impl<I> Member<I>
where
    I: Id,
{
    pub fn new(id: I, name: Name, email: EmailAddress) -> Self {
        Member {
            id,
            created_at: Utc::now(),
            name,
            email,
        }
    }
}

impl<I> UserTrait for Member<I>
where
    I: Id,
{
    type Id = I;
    fn id(&self) -> Self::Id {
        self.id
    }
}

//============== GUEST USER

#[derive(Clone)]
pub struct Guest<I>
where
    I: Id,
{
    pub id: I,
    pub created_at: DateTime<Utc>,
}

impl<I> Guest<I>
where
    I: Id,
{
    pub fn new(id: I) -> Self {
        Guest {
            id,
            created_at: Utc::now(),
        }
    }
}

impl<I> UserTrait for Guest<I>
where
    I: Id,
{
    type Id = I;
    fn id(&self) -> Self::Id {
        self.id
    }
}

//============== ENUM USER allows operating with guest as with mem
#[derive(Clone)]
pub enum User<I>
where
    I: Id,
{
    Guest(Guest<I>),
    Member(Member<I>),
}
