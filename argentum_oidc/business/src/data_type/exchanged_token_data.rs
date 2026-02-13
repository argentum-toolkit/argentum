use argentum_standard_business::data_type::id::Id;

pub struct ExchangedTokenData<I>
where
    I: Id,
{
    pub access_token: String,
    pub user_id: I,
}

impl<I> ExchangedTokenData<I>
where
    I: Id,
{
    pub fn new(access_token: &str, user_id: I) -> Self {
        Self {
            access_token: access_token.to_string(),
            user_id,
        }
    }
}
