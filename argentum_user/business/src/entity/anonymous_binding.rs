use argentum_standard_business::data_type::id::Id;
use chrono::{DateTime, Utc};

pub struct AnonymousBinding<I>
where
    I: Id,
{
    pub user_id: I,
    pub anonymous_id: I,
    pub created_at: DateTime<Utc>,
}

impl<I> AnonymousBinding<I>
where
    I: Id,
{
    pub fn new(user_id: I, anonymous_id: I) -> AnonymousBinding<I> {
        AnonymousBinding {
            user_id,
            anonymous_id,
            created_at: Utc::now(),
        }
    }
}
