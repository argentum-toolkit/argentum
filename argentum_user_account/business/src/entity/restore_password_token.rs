use argentum_standard_business::data_type::id::Id;
use chrono::{DateTime, Duration, Utc};

pub struct RestorePasswordToken {
    pub id: Id,
    pub user_id: Id,
    pub token: String,
    pub created_at: DateTime<Utc>,
}

impl RestorePasswordToken {
    pub fn new(id: Id, user_id: Id, token: String) -> Self {
        RestorePasswordToken {
            id,
            user_id,
            token,
            created_at: Utc::now(),
        }
    }

    pub fn is_expired(&self, ttl: u32) -> bool {
        let expires_at = self.created_at + Duration::seconds(ttl as i64);

        Utc::now().ge(&expires_at)
    }
}
