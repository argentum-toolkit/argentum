use argentum_standard_business::data_type::id::Id;
use chrono::{DateTime, Utc};

pub struct AnonymousBinding {
    pub user_id: Id,
    pub anonymous_id: Id,
    pub created_at: DateTime<Utc>,
}

impl AnonymousBinding {
    pub fn new(user_id: Id, anonymous_id: Id) -> AnonymousBinding {
        AnonymousBinding {
            user_id,
            anonymous_id,
            created_at: Utc::now(),
        }
    }
}
