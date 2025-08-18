use crate::data_type::id::{Id, IdFactory, IntId};
use fastrand;

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
        let id = fastrand::u64(1..=u64::MAX);

        Box::new(IntId::new(id))
    }
}
