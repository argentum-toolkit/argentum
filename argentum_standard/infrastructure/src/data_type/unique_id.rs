use std::fmt::Display;

use argentum_standard_business::data_type::id::{Id, IdFactory};
use uuid::Uuid;

/// UniqueId is Uuid binding for Id trait

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
pub struct UniqueId(Uuid);

impl UniqueId {
    pub fn new() -> Self {
        UniqueId(Uuid::new_v4())
    }

    pub fn as_id<I>(self) -> I
    where
        I: From<UniqueId>,
    {
        I::from(self)
    }
}

impl Id for UniqueId {}

impl Display for UniqueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for UniqueId {
    fn from(value: Uuid) -> Self {
        UniqueId(value)
    }
}

impl From<UniqueId> for Uuid {
    fn from(val: UniqueId) -> Self {
        val.0
    }
}

pub struct UniqueIdFactory {}

impl UniqueIdFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for UniqueIdFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl IdFactory for UniqueIdFactory {
    type Id = UniqueId;

    fn create(&self) -> UniqueId {
        UniqueId(Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::unique_id::UniqueIdFactory;
    use argentum_standard_business::data_type::id::IdFactory;

    #[test]
    fn test_factory_create() {
        let f = UniqueIdFactory {};
        f.create();
    }

    #[test]
    fn test_clone_unique_id() {
        let f = UniqueIdFactory {};
        let id = f.create();

        assert_eq!(id.clone(), id)
    }

    #[test]
    fn test_partial_equal_for_unique_id() {
        let f = UniqueIdFactory {};
        let id = f.create();

        assert_eq!(id.clone(), id)
    }

    #[test]
    fn test_negative_partial_equal_for_unique_id() {
        let f = UniqueIdFactory {};
        let id1 = f.create();
        let id2 = f.create();

        assert_ne!(id1, id2)
    }
}
