use crate::token::MemberGeneratorTrait;

use argentum_standard_business::data_type::id::Id;

pub struct MemberGeneratorMock<I>
where
    I: Id,
{
    _phantom: std::marker::PhantomData<I>,
}

impl<I> MemberGeneratorMock<I>
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

impl<I> MemberGeneratorTrait for MemberGeneratorMock<I>
where
    I: Id,
{
    type Id = I;

    fn generate(&self, _user_id: Self::Id) -> String {
        "Test token".into()
    }
}
