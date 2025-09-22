use std::{fmt::Display, hash::Hash};

pub trait Id: Eq + PartialEq + Clone + Copy + Display + Hash + Send + Sync {}

impl<T> Id for T where T: Eq + PartialEq + Clone + Copy + Display + Hash + Send + Sync {}

pub trait IdFactory: Send + Sync {
    type Id: Id;

    fn create(&self) -> Self::Id;
}

pub type UIntId = u64;
