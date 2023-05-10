use argentum_standard_business::data_type::id::Id;
use argentum_user_business::token::GeneratorTrait;

pub struct TokenGeneratorMock {}

impl TokenGeneratorMock {
    #[allow(clippy::new_without_default)]
    pub fn new() -> TokenGeneratorMock {
        TokenGeneratorMock {}
    }
}

impl GeneratorTrait for TokenGeneratorMock {
    fn generate(&self, _user_id: &Id) -> String {
        "Test token".into()
    }
}
