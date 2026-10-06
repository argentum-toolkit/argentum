use crate::token::GuestGeneratorTrait;
use argentum_standard_business::data_type::id::Id;

pub struct GuestGeneratorMock<I>
where
    I: Id,
{
    _phantom: std::marker::PhantomData<I>,
}

impl<I> GuestGeneratorMock<I>
where
    I: Id,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I> GuestGeneratorTrait for GuestGeneratorMock<I>
where
    I: Id,
{
    type Id = I;

    fn generate(&self, _user_id: Self::Id) -> String {
        "Test token".into()
    }
}
