use crate::value_object::name::Name;
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::IdTrait;
use datetime::LocalDateTime;

pub trait UserTrait {
    fn id(&self) -> Box<dyn IdTrait>;
}

//============== AUTHENTICATED USER

pub struct AuthenticatedUser {
    pub id: Box<dyn IdTrait>,
    pub created_at: LocalDateTime,
    pub name: Name,
    pub email: EmailAddress,
}

impl AuthenticatedUser {
    pub fn new(id: &Box<dyn IdTrait>, name: Name, email: EmailAddress) -> Self {
        AuthenticatedUser {
            id: id.clone(),
            created_at: LocalDateTime::now(),
            name,
            email,
        }
    }
}

impl UserTrait for AuthenticatedUser {
    fn id(&self) -> Box<dyn IdTrait> {
        self.id.clone()
    }
}

//============== ANONYMOUS USER

pub struct AnonymousUser {
    pub id: Box<dyn IdTrait>,
    pub created_at: LocalDateTime,
}

impl AnonymousUser {
    pub fn new(id: &Box<dyn IdTrait>) -> Self {
        AnonymousUser {
            id: id.clone(),
            created_at: LocalDateTime::now(),
        }
    }
}

impl UserTrait for AnonymousUser {
    fn id(&self) -> Box<dyn IdTrait> {
        self.id.clone()
    }
}

//============== ENUM USER allows to operate with anonymous as with authenticated users
pub enum User {
    Anonymous(AnonymousUser),
    Authenticated(AuthenticatedUser),
}
