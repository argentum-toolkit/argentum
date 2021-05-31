use crate::value_object::name::Name;
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use datetime::LocalDateTime;

pub trait UserTrait {
    fn id(&self) -> Id;
}

//============== AUTHENTICATED USER

pub struct AuthenticatedUser {
    pub id: Id,
    pub created_at: LocalDateTime,
    pub name: Name,
    pub email: EmailAddress,
}

impl AuthenticatedUser {
    pub fn new(id: &Id, name: Name, email: EmailAddress) -> Self {
        AuthenticatedUser {
            id: id.clone(),
            created_at: LocalDateTime::now(),
            name,
            email,
        }
    }
}

impl UserTrait for AuthenticatedUser {
    fn id(&self) -> Id {
        self.id.clone()
    }
}

//============== ANONYMOUS USER

pub struct AnonymousUser {
    pub id: Id,
    pub created_at: LocalDateTime,
}

impl AnonymousUser {
    pub fn new(id: &Id) -> Self {
        AnonymousUser {
            id: id.clone(),
            created_at: LocalDateTime::now(),
        }
    }
}

impl UserTrait for AnonymousUser {
    fn id(&self) -> Id {
        self.id.clone()
    }
}

//============== ENUM USER allows to operate with anonymous as with authenticated users
pub enum User {
    Anonymous(AnonymousUser),
    Authenticated(AuthenticatedUser),
}
