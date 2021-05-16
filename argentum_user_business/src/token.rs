use argentum_standard_business::data_type::id::IdTrait;

pub trait GeneratorTrait {
    fn generate(&self, user_id: &Box<dyn IdTrait>) -> String;
}
