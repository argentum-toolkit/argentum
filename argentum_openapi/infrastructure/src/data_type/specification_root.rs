use crate::data_type::path::Path;
use crate::data_type::{Components, ExternalDocs, Info, Operation};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpecificationRoot {
    pub openapi: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocs>,

    pub info: Info,

    pub paths: BTreeMap<String, Path>,

    pub components: Components,
    //TODO: servers https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#licenseObject
    //TODO: security
    //TODO: tags
}

impl SpecificationRoot {
    pub fn new_empty() -> Self {
        Self {
            openapi: "".to_string(),
            paths: Default::default(),
            components: Components {
                request_bodies: Default::default(),
                schemas: Default::default(),
                responses: Default::default(),
            },
            info: Info {
                title: Default::default(),
                version: Default::default(),
                description: None,
                terms_of_service: None,
                contact: None,
                licence: None,
            },
            external_docs: None,
        }
    }

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
