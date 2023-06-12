use argentum_standard_business::data_type::id::Id;

pub trait GeneratorTrait: Send + Sync {
    fn generate(&self, user_id: &Id) -> String;
}
