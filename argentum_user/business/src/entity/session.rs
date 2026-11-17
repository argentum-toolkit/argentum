use argentum_standard_business::data_type::id::Id;
use jiff::Timestamp;

pub struct Session<I>
where
    I: Id,
{
    pub id: I,
    pub user_id: I,
    pub created_at: Timestamp,
    pub token: String,
    //device_information???
}

impl<I> Session<I>
where
    I: Id,
{
    pub fn new(id: I, user_id: I, token: &str) -> Self {
        Session {
            id,
            user_id,
            created_at: Timestamp::now(),
            token: token.into(),
        }
    }
}
