pub type AuthHash = String;
pub type UserId = uuid::Uuid;

pub enum Security {
    Anonymous(AuthHash),
    Authenticated(AuthHash, UserId),
}
