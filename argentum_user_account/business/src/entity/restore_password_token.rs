use argentum_standard_business::data_type::id::Id;
use jiff::{Span, Timestamp};

pub struct RestorePasswordToken<I>
where
    I: Id,
{
    pub id: I,
    pub user_id: I,
    pub token: String,
    pub created_at: Timestamp,
}

impl<I> RestorePasswordToken<I>
where
    I: Id,
{
    pub fn new(id: I, user_id: I, token: &str) -> Self {
        RestorePasswordToken {
            id,
            user_id,
            token: token.into(),
            created_at: Timestamp::now(),
        }
    }

    pub fn is_expired(&self, ttl: u32) -> bool {
        let expires_at = self.created_at + Span::new().seconds(ttl as i64);

        Timestamp::now().ge(&expires_at)
    }
}
