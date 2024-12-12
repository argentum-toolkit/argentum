use argentum_openapi_infrastructure::data_type::{RefOrObject, Schema, SpecificationRoot};

pub struct SchemaExtractor {}

impl SchemaExtractor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract(&self, ref_or: &RefOrObject<Schema>, spec: &SpecificationRoot) -> Schema {
        match ref_or {
            RefOrObject::Ref(r) => {
                let schema_name = r
                    .reference
                    .clone()
                    .split('/')
                    .last()
                    .unwrap_or_else(|| {
                        panic!(
                            "Wrong schema href {}. Expected: `#/components/schemas/{{name}}`",
                            r.reference
                        )
                    })
                    .to_string();

                spec.components.schemas[&schema_name].clone()
            }
            RefOrObject::Object(s) => s.clone(),
        }
    }
}
