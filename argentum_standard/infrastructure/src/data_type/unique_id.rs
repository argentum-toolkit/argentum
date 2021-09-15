use argentum_standard_business::data_type::id::{Id, IdFactory, IdTrait, Printable};
use std::any::Any;
use uuid::Uuid;

/// Unique id is Uuid binding for IdTrait
pub struct UniqueId {
    pub value: Uuid,
}

impl IdTrait for UniqueId {
    fn id_eq(&self, other: &dyn Any) -> bool {
        other
            .downcast_ref::<Self>()
            .map_or(false, |id| id.value == self.value)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn id_clone(&self) -> Id {
        Box::new(UniqueId::new(self.value))
    }
}

impl Printable for UniqueId {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl UniqueId {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(id: Uuid) -> impl IdTrait {
        UniqueId { value: id }
    }
}

pub struct UniqueIdFactory {}

impl UniqueIdFactory {
    pub fn new() -> UniqueIdFactory {
        UniqueIdFactory {}
    }

    pub fn uuid_to_id(&self, uuid: Uuid) -> Id {
        Box::new(UniqueId::new(uuid))
    }

    pub fn id_to_uuid(&self, id: Id) -> Uuid {
        id.as_any()
            .downcast_ref::<UniqueId>()
            .map(|id| id.value)
            .unwrap()
    }
}

impl Default for UniqueIdFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl IdFactory for UniqueIdFactory {
    fn create(&self) -> Id {
        Box::new(UniqueId::new(Uuid::new_v4()))
    }
}

pub fn new_unique_id() -> Id {
    Box::new(UniqueId::new(Uuid::new_v4()))
}

#[cfg(test)]
mod tests {
    use crate::data_type::unique_id::{new_unique_id, UniqueIdFactory};
    use argentum_standard_business::data_type::id::IdFactory;

    #[test]
    fn test_new_unique_id() {
        new_unique_id();
    }

    #[test]
    fn test_factory_create() {
        let f = UniqueIdFactory {};
        f.create();
    }

    #[test]
    fn test_clone_unique_id() {
        let f = UniqueIdFactory {};
        let id = f.create();

        assert_eq!(id.clone().to_string(), id.to_string())
    }

    #[test]
    fn test_partial_equal_for_unique_id() {
        let f = UniqueIdFactory {};
        let id = f.create();

        assert_eq!(id.clone().to_string(), id.to_string())
    }

    #[test]
    fn test_negative_partial_equal_for_unique_id() {
        let f = UniqueIdFactory {};
        let id1 = f.create();
        let id2 = f.create();

        assert_ne!(id1.to_string(), id2.to_string())
    }

    #[test]
    fn test_printable_for_unique_id() {
        let f = UniqueIdFactory {};
        let id = f.create();

        assert_eq!(id.to_string().len(), 36)
    }
}
