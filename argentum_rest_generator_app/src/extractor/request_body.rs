use argentum_openapi_infrastructure::data_type::{
    Operation, RefOrObject, RequestBody, SpecificationRoot,
};

pub struct RequestBodyExtractor {}

impl RequestBodyExtractor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract(&self, operation: &Operation, spec: &SpecificationRoot) -> Option<RequestBody> {
        let ref_or = operation.clone().request_body?;

        match ref_or {
            RefOrObject::Ref(r) => {
                let parts = r.reference.split("#/").collect::<Vec<_>>();

                if parts.clone().len() != 2 {
                    panic!("Wrong format of reference {}", r.reference)
                }

                let _file_path = parts
                    .first()
                    .unwrap_or_else(|| panic!("Wrong file path of reference {}", r.reference));

                let component_path = parts
                    .last()
                    .unwrap_or_else(|| panic!("Wrong component path of reference {}", r.reference));

                let component_parts = component_path.split('/').collect::<Vec<_>>();

                if component_parts.clone().len() != 3
                    || component_parts[0] != "components"
                    || component_parts[1] != "requestBodies"
                {
                    panic!(
                        "Wrong component path {}. Expected: `#/components/requestBodies/{{name}}`",
                        component_path
                    )
                }

                let component_name = component_parts.last().unwrap_or_else(|| {
                    panic!(
                        "Wrong component path {}. Expected: `#/components/requestBodies/{{name}}`",
                        component_path
                    )
                });

                Some(spec.components.request_bodies[&component_name.to_string()].clone())
            }

            RefOrObject::Object(request_body) => Some(request_body),
        }
    }
}
