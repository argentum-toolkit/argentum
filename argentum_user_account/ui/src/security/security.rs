pub type AuthHash = String;
pub type UserId = uuid::Uuid;

pub enum Security {
    Guest(AuthHash),
    Member(AuthHash, UserId),
}
