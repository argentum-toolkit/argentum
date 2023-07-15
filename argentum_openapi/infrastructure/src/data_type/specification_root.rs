use crate::data_type::path::Path;
use crate::data_type::{Components, Operation};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpecificationRoot {
    pub openapi: String,
    pub paths: BTreeMap<String, Path>,
    pub components: Components,
}

impl SpecificationRoot {
    pub fn operations(&self) -> Vec<Operation> {
        let mut operations: Vec<Operation> = vec![];

        for (_, path) in self.paths.clone().into_iter() {
            for (_, operation) in path.operations.into_iter() {
                operations.push(operation.clone());
            }
        }

        operations.sort_by(|a, b| a.operation_id.cmp(&b.operation_id));

        operations
    }
}
