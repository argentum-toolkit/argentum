use argentum_standard_business::data_type::id::Id;
use chrono::{DateTime, Utc};

pub struct Credential<I>
where
    I: Id,
{
    pub external_id: String,

    pub user_id: I,

    pub provider: String,

    pub created_at: DateTime<Utc>,
}

impl<I> Credential<I>
where
    I: Id,
{
    pub fn new(external_id: &str, user_id: I, provider: &str, created_at: DateTime<Utc>) -> Self {
        Self {
            external_id: external_id.into(),
            user_id,
            provider: provider.into(),
            created_at,
        }
    }
}
