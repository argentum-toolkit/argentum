use argentum_standard_business::data_type::id::Id;
use chrono::{DateTime, Utc};

pub struct GuestBinding<I>
where
    I: Id,
{
    pub member_id: I,
    pub guest_id: I,
    pub created_at: DateTime<Utc>,
}

impl<I> GuestBinding<I>
where
    I: Id,
{
    pub fn new(member_id: I, guest_id: I) -> GuestBinding<I> {
        GuestBinding {
            member_id,
            guest_id,
            created_at: Utc::now(),
        }
    }
}
