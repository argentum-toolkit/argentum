use std::any::Any;
use std::hash::{Hash, Hasher};

pub trait Printable {
    fn to_string(&self) -> String;
}

pub trait IdTrait: Any + Printable {
    fn id_eq(&self, other: &dyn Any) -> bool;
    //Convert to Any for comparison
    fn as_any(&self) -> &dyn Any;

    fn id_clone(&self) -> Box<dyn IdTrait>;
}

impl PartialEq for Box<dyn IdTrait> {
    fn eq(&self, other: &Box<dyn IdTrait>) -> bool {
        self.id_eq(other.as_any())
    }
}

impl Eq for Box<dyn IdTrait> {}

impl Hash for Box<dyn IdTrait> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

impl Clone for Box<dyn IdTrait> {
    fn clone(&self) -> Box<dyn IdTrait> {
        self.id_clone()
    }
}

pub trait IdFactory {
    fn create(&self) -> Box<dyn IdTrait>;
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
    fn id_clone(&self) -> Box<dyn IdTrait> {
        Box::new(IntId::new(self.value))
    }
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
