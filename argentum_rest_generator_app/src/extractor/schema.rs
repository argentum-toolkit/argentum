use argentum_openapi_infrastructure::data_type::{
    RefOrObject, Reference, Schema, SpecificationRoot,
};

pub struct SchemaExtractor {}

impl SchemaExtractor {
    pub fn new() -> Self {
        Self {}
    }

    fn extract_name(&self, reference: &Reference) -> String {
        reference
            .reference
            .clone()
            .split('/')
            .last()
            .unwrap_or_else(|| {
                panic!(
                    "Wrong schema href {}. Expected: `#/components/schemas/{{name}}`",
                    reference.reference
                )
            })
            .to_string()
    }

    pub fn extract(&self, ref_or: &RefOrObject<Schema>, spec: &SpecificationRoot) -> Schema {
        match ref_or {
            RefOrObject::Ref(r) => {
                let schema_name = self.extract_name(r);
                spec.components.schemas[&schema_name].clone()
            }
            RefOrObject::Object(s) => s.clone(),
        }
    }

    pub fn extract_ref_with_name(
        &self,
        ref_or: &RefOrObject<Schema>,
        spec: &SpecificationRoot,
    ) -> Option<(String, Schema)> {
        match ref_or {
            RefOrObject::Ref(r) => {
                let schema_name = self.extract_name(r);
                let s = spec.components.schemas[&schema_name].clone();

                Some((schema_name, s))
            }
            RefOrObject::Object(_) => None,
        }
    }
}
