use crate::generator::OasLoader;
use argentum_openapi_infrastructure::data_type::{
    ComponentRef, RefOrObject, RequestBody, Response, Schema, SchemaType, SpecificationRoot,
};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

pub struct Combiner {
    loader: Arc<OasLoader>,
}

impl Combiner {
    pub fn new(loader: Arc<OasLoader>) -> Self {
        Self { loader }
    }

    fn collect_request_body(
        &self,
        body: &mut RequestBody,
        current_file_path: PathBuf,
    ) -> (SpecificationRoot, RequestBody) {
        let mut to_spec = SpecificationRoot::new_empty();

        for (_content_type, media_type) in &mut body.content {
            let ref_or_schema = &mut media_type.schema;

            self.collect_ref_to_schema(ref_or_schema, &mut to_spec, current_file_path.clone());
        }

        (to_spec, body.clone())
    }

    fn collect_response(
        &self,
        response: &mut Response,
        current_file_path: PathBuf,
    ) -> (SpecificationRoot, Response) {
        let mut to_spec = SpecificationRoot::new_empty();

        for (_content_type, media_type) in &mut response.content {
            let ref_or_schema = &mut media_type.schema;

            self.collect_ref_to_schema(ref_or_schema, &mut to_spec, current_file_path.clone());
        }

        (to_spec, response.clone())
    }

    fn collect_schema(
        &self,
        schema: &mut Schema,
        current_file_path: PathBuf,
    ) -> (SpecificationRoot, Schema) {
        let mut spec = SpecificationRoot::new_empty();

        self.collect_schema_properties(schema, current_file_path, &mut spec);

        (spec, schema.clone())
    }

    fn collect_schema_properties(
        &self,
        schema: &mut Schema,
        current_file_path: PathBuf,
        to_spec: &mut SpecificationRoot,
    ) {
        match schema.schema_type {
            Some(SchemaType::Array) => {}
            Some(SchemaType::Object) => {
                if let Some(properties) = schema.properties.as_mut() {
                    self.collect_properties(properties, to_spec, current_file_path);
                }
            }
            Some(_) => {
                //TODO: warning
            }
            None => {
                //TODO: warning
            }
        }
    }
    fn collect_ref_to_schema(
        &self,
        property: &mut RefOrObject<Schema>,
        to_spec: &mut SpecificationRoot,
        current_file_path: PathBuf,
    ) {
        if let RefOrObject::Ref(r) = property {
            let component_ref = ComponentRef::from(r.reference.clone());
            if !component_ref.is_schema() {
                panic!(
                    "Wrong reference to schema component: `{}`",
                    r.reference.clone()
                );
            }

            if let Some(file_path) = component_ref.file_path {
                let dir = current_file_path.parent().unwrap();
                let dir = dir.to_str().unwrap();

                let inner_file_path = format!("{}/{}", dir.to_string().clone(), file_path);
                //load from filesystem
                let (include_spec, _include_spec_file_path) =
                    self.loader.load(inner_file_path.clone());

                let component: Option<&Schema> = include_spec
                    .components
                    .schemas
                    .get(component_ref.component_name.as_str());
                match component {
                    None => {
                        panic!(
                            "Schema #/components/schemas/{} is not found",
                            component_ref.component_name.clone()
                        )
                    }
                    Some(s) => {
                        let c_name = component_ref.component_name;

                        let reference = format!("#/components/schemas/{}", c_name);
                        r.reference = reference;

                        let ss: &mut Schema = &mut s.clone();
                        self.collect_schema_properties(ss, inner_file_path.into(), to_spec);

                        to_spec.components.schemas.insert(c_name, ss.clone());
                    }
                }
            }
        }
    }

    fn collect_ref_to_request_body(
        &self,
        property: &mut RefOrObject<RequestBody>,
        to_spec: &mut SpecificationRoot,
        current_file_path: PathBuf,
    ) {
        if let RefOrObject::Ref(r) = property {
            let component_ref = ComponentRef::from(r.reference.clone());
            if !component_ref.is_request_body() {
                panic!(
                    "Wrong reference to RequestBody component: `{}`",
                    r.reference.clone()
                );
            }

            if let Some(file_path) = component_ref.file_path {
                let dir = current_file_path.parent().unwrap();
                let dir = dir.to_str().unwrap();

                let inner_file_path = format!("{}/{}", dir.to_string().clone(), file_path);
                //load from filesystem
                let (include_spec, _include_spec_file_path) =
                    self.loader.load(inner_file_path.clone());

                let component: Option<&RequestBody> = include_spec
                    .components
                    .request_bodies
                    .get(component_ref.component_name.as_str());
                match component {
                    None => {
                        panic!(
                            "Request body #/components/requestBodies/{} is not found",
                            component_ref.component_name.clone()
                        )
                    }
                    Some(s) => {
                        let b_name = component_ref.component_name;

                        let reference = format!("#/components/requestBodies/{}", b_name);
                        r.reference = reference;

                        let b: &mut RequestBody = &mut s.clone();

                        let (res_spec, res_body) =
                            self.collect_request_body(b, inner_file_path.into());

                        // collect_schema_properties(ss, inner_file_path.into(), to_spec);

                        for (n, s) in res_spec.components.schemas {
                            to_spec.components.schemas.insert(n, s.clone());
                        }

                        to_spec.components.request_bodies.insert(b_name, res_body);
                    }
                }
            }
        }
    }

    fn collect_ref_to_response(
        &self,
        property: &mut RefOrObject<Response>,
        to_spec: &mut SpecificationRoot,
        current_file_path: PathBuf,
    ) {
        if let RefOrObject::Ref(r) = property {
            let component_ref = ComponentRef::from(r.reference.clone());
            if !component_ref.is_response() {
                panic!(
                    "Wrong reference to response component: `{}`",
                    r.reference.clone()
                );
            }

            if let Some(file_path) = component_ref.file_path {
                let dir = current_file_path.parent().unwrap();
                let dir = dir.to_str().unwrap();

                let inner_file_path = format!("{}/{}", dir.to_string().clone(), file_path);
                //load from filesystem
                let (include_spec, _include_spec_file_path) =
                    self.loader.load(inner_file_path.clone());

                let component: Option<&Response> = include_spec
                    .components
                    .responses
                    .get(component_ref.component_name.as_str());
                match component {
                    None => {
                        panic!(
                            "Response #/components/responses/{} is not found",
                            component_ref.component_name.clone()
                        )
                    }
                    Some(s) => {
                        let b_name = component_ref.component_name;

                        let reference = format!("#/components/responses/{}", b_name);
                        r.reference = reference;

                        let resp: &mut Response = &mut s.clone();

                        let (res_spec, res_resp) =
                            self.collect_response(resp, inner_file_path.into());

                        for (n, s) in res_spec.components.schemas {
                            to_spec.components.schemas.insert(n, s.clone());
                        }

                        to_spec.components.responses.insert(b_name, res_resp);
                    }
                }
            }
        }
    }

    fn collect_properties(
        &self,
        properties: &mut BTreeMap<String, RefOrObject<Schema>>,
        to_spec: &mut SpecificationRoot,
        current_file_path: PathBuf,
    ) {
        for (_name, property) in properties.into_iter() {
            self.collect_ref_to_schema(property, to_spec, current_file_path.clone());
        }
    }

    pub fn combine(&self, file_path: String) -> SpecificationRoot {
        let (mut spec, current_file_path) = self.loader.load(file_path);
        let mut res_spec = SpecificationRoot::new_empty();

        res_spec.openapi = spec.openapi.clone();
        res_spec.external_docs = spec.external_docs.clone();
        res_spec.info = spec.info.clone();
        res_spec.security = spec.security.clone();
        res_spec.tags = spec.tags.clone();
        res_spec.servers = spec.servers.clone();

        for (body_name, body) in &mut spec.components.request_bodies {
            let (body_spec, updated_body) =
                self.collect_request_body(body, current_file_path.clone());

            for (n, s) in body_spec.components.schemas {
                res_spec.components.schemas.insert(n, s.clone());
            }

            res_spec
                .components
                .request_bodies
                .insert(body_name.clone(), updated_body);
        }

        for (response_name, response) in &mut spec.components.responses {
            let (body_spec, updated_response) =
                self.collect_response(response, current_file_path.clone());

            for (n, s) in body_spec.components.schemas {
                res_spec.components.schemas.insert(n, s.clone());
            }

            res_spec
                .components
                .responses
                .insert(response_name.clone(), updated_response);
        }

        for (schema_name, schema) in &mut spec.components.schemas {
            let (schemas_spec, updated_schema) =
                self.collect_schema(schema, current_file_path.clone());

            for (n, s) in schemas_spec.components.schemas {
                res_spec.components.schemas.insert(n, s.clone());
            }

            res_spec
                .components
                .schemas
                .insert(schema_name.clone(), updated_schema);
        }

        for (uri, path) in &mut spec.paths {
            for (_method_name, operation) in &mut path.operations {
                if let Some(ref_or_schema) = &mut operation.request_body {
                    self.collect_ref_to_request_body(
                        ref_or_schema,
                        &mut res_spec,
                        current_file_path.clone(),
                    );
                }

                for (_response_code, ref_or_response) in &mut operation.responses {
                    self.collect_ref_to_response(
                        ref_or_response,
                        &mut res_spec,
                        current_file_path.clone(),
                    );
                }
            }

            res_spec.paths.insert(uri.clone(), path.clone());
        }

        res_spec.components.security_schemes = spec.components.security_schemes.clone();

        return res_spec;
    }
}
