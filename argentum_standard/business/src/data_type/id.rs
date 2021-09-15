use std::any::Any;
use std::hash::{Hash, Hasher};

pub trait Printable {
    fn to_string(&self) -> String;
}

pub trait IdTrait: Any + Printable + Sync + Send {
    fn id_eq(&self, other: &dyn Any) -> bool;
    //Convert to Any for comparison
    fn as_any(&self) -> &dyn Any;

    fn id_clone(&self) -> Id;

    // fn as_bytes(&self) -> &[u8];
}

pub type Id = Box<dyn IdTrait>;

impl PartialEq for dyn IdTrait {
    fn eq(&self, other: &dyn IdTrait) -> bool {
        self.id_eq(other.as_any())
    }
}

impl Eq for dyn IdTrait {}

impl Hash for dyn IdTrait {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

impl Clone for Id {
    fn clone(&self) -> Id {
        self.id_clone()
    }
}

pub trait IdFactory: Send + Sync {
    fn create(&self) -> Id;
}

pub struct IntId {
    value: u64,
}

impl IdTrait for IntId {
    fn id_eq(&self, other: &dyn Any) -> bool {
        other
            .downcast_ref::<Self>()
            .map_or(false, |id| id.value == self.value)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn id_clone(&self) -> Id {
        Box::new(IntId::new(self.value))
    }
    // fn as_bytes(&self) -> &[u8] {
    //     &self.value.to_be_bytes()
    // }
}

impl Printable for IntId {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl IntId {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(id: u64) -> impl IdTrait {
        IntId { value: id }
    }
}
