use argentum_standard_business::data_type::id::Id;
use jiff::Timestamp;

pub struct GuestBinding<I>
where
    I: Id,
{
    pub member_id: I,
    pub guest_id: I,
    pub created_at: Timestamp,
}

impl<I> GuestBinding<I>
where
    I: Id,
{
    pub fn new(member_id: I, guest_id: I) -> GuestBinding<I> {
        GuestBinding {
            member_id,
            guest_id,
            created_at: Timestamp::now(),
        }
    }
}
