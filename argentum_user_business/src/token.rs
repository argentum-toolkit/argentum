use argentum_standard_business::data_type::id::Id;

pub trait GeneratorTrait {
    fn generate(&self, user_id: &Id) -> String;
}
