use argentum_openapi_infrastructure::data_type::{
    Operation, RefOrObject, RequestBody, SpecificationRoot,
};

pub struct RequestBodyExtractor {}

impl RequestBodyExtractor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract(
        &self,
        operation: &Operation,
        spec: &SpecificationRoot,
    ) -> Result<Option<RequestBody>, String> {
        let ref_or = match operation.clone().request_body {
            Some(r) => r,
            None => return Ok(None),
        };

        match ref_or {
            RefOrObject::Ref(r) => {
                let parts = r.reference.split("#/").collect::<Vec<_>>();

                if parts.clone().len() != 2 {
                    return Err(format!("Wrong format of reference {}", r.reference).into());
                }

                let _file_path = parts
                    .first()
                    .ok_or(format!("Wrong file path of reference {}", r.reference))?;

                let component_path = parts
                    .last()
                    .ok_or(format!("Wrong component path of reference {}", r.reference))?;

                let component_parts = component_path.split('/').collect::<Vec<_>>();

                if component_parts.clone().len() != 3
                    || component_parts[0] != "components"
                    || component_parts[1] != "requestBodies"
                {
                    return Err(format!(
                        "Wrong component path {component_path}. Expected: `#/components/requestBodies/{{name}}`"
                    ).into());
                }

                let component_name = component_parts.last().ok_or(format!(
                    "Wrong component path {}. Expected: `#/components/requestBodies/{{name}}`",
                    component_path
                ))?;

                Ok(Some(
                    spec.components.request_bodies[&component_name.to_string()].clone(),
                ))
            }

            RefOrObject::Object(request_body) => Ok(Some(request_body)),
        }
    }
}
