use argentum_standard_business::data_type::id::Id;

pub trait GeneratorTrait: Send + Sync {
    type Id: Id;

    fn generate(&self, user_id: Self::Id) -> String;
}
