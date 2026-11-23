use argentum_openapi_infrastructure::data_type::{
    RefOrObject, Reference, Schema, SpecificationRoot,
};

pub struct SchemaExtractor {}

impl SchemaExtractor {
    pub fn new() -> Self {
        Self {}
    }

    fn extract_name(&self, reference: &Reference) -> Result<String, String> {
        reference
            .reference
            .clone()
            .split('/')
            .next_back()
            .map(|n| n.to_string())
            .ok_or(format!(
                "Wrong schema href {}. Expected: `#/components/schemas/{{name}}`",
                reference.reference
            ))
    }

    pub fn extract(
        &self,
        ref_or: &RefOrObject<Schema>,
        spec: &SpecificationRoot,
    ) -> Result<Schema, String> {
        match ref_or {
            RefOrObject::Ref(r) => {
                let schema_name = self.extract_name(r)?;
                Ok(spec.components.schemas[&schema_name].clone())
            }
            RefOrObject::Object(s) => Ok(s.clone()),
        }
    }

    pub fn extract_ref_with_name(
        &self,
        ref_or: &RefOrObject<Schema>,
        spec: &SpecificationRoot,
    ) -> Result<Option<(String, Schema)>, String> {
        match ref_or {
            RefOrObject::Ref(r) => {
                let schema_name = self.extract_name(r)?;
                let s = spec.components.schemas[&schema_name].clone();

                Ok(Some((schema_name, s)))
            }
            RefOrObject::Object(_) => Ok(None),
        }
    }
}
