use crate::data_type::id::{IdFactory, UIntId};
use fastrand;

#[derive(Default)]
pub struct IdFactoryMock {}

impl IdFactoryMock {
    pub fn new() -> IdFactoryMock {
        IdFactoryMock {}
    }
}

impl IdFactory for IdFactoryMock {
    type Id = UIntId;

    fn create(&self) -> self::UIntId {
        fastrand::u64(1..=u64::MAX)
    }
}
