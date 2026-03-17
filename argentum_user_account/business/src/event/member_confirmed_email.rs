use std::any::Any;

use argentum_event_business::Event;
use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;

pub struct MemberConfirmedEmail<I>
where
    I: Id,
{
    pub member_id: I,
}

impl<I> MemberConfirmedEmail<I>
where
    I: Id,
{
    pub fn new(member_id: I) -> Self {
        Self { member_id }
    }
}

#[async_trait]
impl<I: Send + Sync + 'static> Event for MemberConfirmedEmail<I>
where
    I: Id,
{
    fn name(&self) -> &'static str {
        "AG:UA:MemberConfirmedEmail"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
