use argentum_standard_business::data_type::id::{Id, IdFactory, IntId};

use rand::Rng;

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
    fn create(&self) -> Id {
        let mut rng = rand::thread_rng();
        let id = rng.gen_range(1..u64::MAX);

        Box::new(IntId::new(id))
    }
}
