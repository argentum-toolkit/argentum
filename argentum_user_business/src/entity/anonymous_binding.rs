use argentum_standard_business::data_type::id::IdTrait;
use datetime::LocalDateTime;

pub struct AnonymousBinding {
    pub user_id: Box<dyn IdTrait>,
    pub anonymous_id: Box<dyn IdTrait>,
    pub created_at: LocalDateTime,
}

impl AnonymousBinding {
    pub fn new(user_id: Box<dyn IdTrait>, anonymous_id: Box<dyn IdTrait>) -> AnonymousBinding {
        AnonymousBinding {
            user_id,
            anonymous_id,
            created_at: LocalDateTime::now(),
        }
    }
}
