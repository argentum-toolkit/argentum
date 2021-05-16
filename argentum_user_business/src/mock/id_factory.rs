use argentum_standard_business::data_type::id::{IdFactory, IdTrait, IntId};

pub struct IdFactoryMock {}

impl IdFactoryMock {
    pub fn new() -> IdFactoryMock {
        IdFactoryMock {}
    }
}

impl Default for IdFactoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl IdFactory for IdFactoryMock {
    fn create(&self) -> Box<dyn IdTrait> {
        Box::new(IntId::new(123))
    }
}
