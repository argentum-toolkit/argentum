use argentum_standard_business::data_type::id::Id;
use datetime::LocalDateTime;

pub struct AnonymousBinding {
    pub user_id: Id,
    pub anonymous_id: Id,
    pub created_at: LocalDateTime,
}

impl AnonymousBinding {
    pub fn new(user_id: Id, anonymous_id: Id) -> AnonymousBinding {
        AnonymousBinding {
            user_id,
            anonymous_id,
            created_at: LocalDateTime::now(),
        }
    }
}
