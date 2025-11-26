use argentum_standard_business::data_type::id::Id;
use chrono::{DateTime, Utc};

pub struct GuestSession<I>
where
    I: Id,
{
    pub id: I,
    pub user_id: I,
    pub created_at: DateTime<Utc>,
    pub token: String,
    //device_information???
}

impl<I> GuestSession<I>
where
    I: Id,
{
    pub fn new(id: I, user_id: I, token: &str) -> Self {
        GuestSession {
            id,
            user_id,
            created_at: Utc::now(),
            token: token.into(),
        }
    }
}
