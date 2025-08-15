use crate::generator::OasLoader;
use argentum_log_business::LoggerTrait;
use argentum_openapi_infrastructure::data_type::{
    ComponentRef, RefOrObject, RequestBody, Response, Schema, SchemaType, SpecificationRoot,
};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::{Arc, RwLock};

pub struct Combiner<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    loader: Arc<OasLoader<L>>,
    combined_schemas: RwLock<HashMap<String, bool>>,
}

impl<L> Combiner<L>
where
    L: LoggerTrait,
{
    pub fn new(logger: Arc<L>, loader: Arc<OasLoader<L>>) -> Self {
        Self {
            logger,
            loader,
            combined_schemas: RwLock::new(HashMap::<String, bool>::new()),
        }
    }

    fn collect_request_body(
        &self,
        body: &mut RequestBody,
        current_file_path: &str,
    ) -> Result<(SpecificationRoot, RequestBody), String> {
        let mut to_spec = SpecificationRoot::new_empty();

        for media_type in body.content.values_mut() {
            let ref_or_schema = &mut media_type.schema;

            self.collect_ref_to_schema(ref_or_schema, &mut to_spec, current_file_path)?;
        }

        Ok((to_spec, body.clone()))
    }

    fn collect_response(
        &self,
        response: &mut Response,
        current_file_path: &str,
    ) -> Result<(SpecificationRoot, Response), String> {
        let mut to_spec = SpecificationRoot::new_empty();

        for media_type in response.content.values_mut() {
            let ref_or_schema = &mut media_type.schema;

            self.collect_ref_to_schema(ref_or_schema, &mut to_spec, current_file_path)?;
        }

        Ok((to_spec, response.clone()))
    }

    fn collect_schema(
        &self,
        schema: &mut Schema,
        current_file_path: &str,
    ) -> Result<(SpecificationRoot, Schema), String> {
        let mut spec = SpecificationRoot::new_empty();

        self.collect_schema_properties(schema, current_file_path, &mut spec)?;

        Ok((spec, schema.clone()))
    }

    fn collect_schema_properties(
        &self,
        schema: &mut Schema,
        current_file_path: &str,
        to_spec: &mut SpecificationRoot,
    ) -> Result<(), String> {
        match schema.schema_type {
            Some(SchemaType::Array) => match &mut *schema.items {
                None => {
                    self.logger
                        .warning("The items keyword is required in arrays");
                }
                Some(items) => {
                    self.collect_ref_to_schema(items, to_spec, current_file_path)?;
                }
            },
            Some(SchemaType::Object) => match *schema.additional_properties.clone() {
                Some(mut additional) => {
                    self.collect_ref_to_schema(&mut additional, to_spec, current_file_path)?;
                }
                None => {
                    if let Some(properties) = schema.properties.as_mut() {
                        self.collect_properties(properties, to_spec, current_file_path)?;
                    }
                }
            },
            Some(_) => {
                self.logger.warning(format!(
                    "Schema type is not supported by combiner. Type: {:?}",
                    schema.schema_type
                ));
            }
            None => {
                self.logger.warning(format!(
                    "Empty schema type is not supported by combiner. File: `{:?}`",
                    current_file_path
                ));
            } //TODO: add support of empty types
        }

        Ok(())
    }

    fn collect_ref_to_schema(
        &self,
        property: &mut RefOrObject<Schema>,
        to_spec: &mut SpecificationRoot,
        current_file_path: &str,
    ) -> Result<(), String> {
        if let RefOrObject::Ref(r) = property {
            let component_ref = ComponentRef::try_from(r.reference.clone())?;
            if !component_ref.is_schema() {
                return Err(format!(
                    "Wrong reference to schema component: `{}`",
                    r.reference.clone()
                ));
            }

            let component_name = component_ref.component_name;

            let reference = format!("#/components/schemas/{component_name}");
            r.reference = reference;

            if let Some(file_path) = component_ref.file_path {
                let dir_res: Result<&str, String> = match Path::new(current_file_path).parent() {
                    Some(d) => match d.to_str() {
                        Some(dd) => Ok(dd),
                        None => Err("Can't get parent as ad dir for file path".into()),
                    },
                    None => Err("Can't get parent dir for file path".into()),
                };

                let dir = dir_res?;

                let inner_file_path = format!("{}/{}", dir.to_string().clone(), file_path);
                let hash_key = format!("{inner_file_path}#{component_name}");
                if self
                    .combined_schemas
                    .read()
                    .map_err(|e| format!("Lock poisoned while reading combined_schemas: {e}"))?
                    .contains_key(&hash_key)
                {
                    self.logger.info(format!(
                        "Schema `{component_name}` already loaded from file `{inner_file_path}`"
                    ));
                } else {
                    self.combined_schemas
                        .write()
                        .map_err(|e| format!("Lock poisoned while writing combined_schemas: {e}"))?
                        .insert(hash_key, true);

                    //load from filesystem
                    let include_spec = self.loader.load(&inner_file_path)?;

                    let component: Option<&Schema> =
                        include_spec.components.schemas.get(component_name.as_str());
                    match component {
                        None => {
                            return Err(format!(
                                "Schema #/components/schemas/{} is not found",
                                component_name.clone()
                            ));
                        }
                        Some(s) => {
                            let ss: &mut Schema = &mut s.clone();
                            self.collect_schema_properties(ss, &inner_file_path, to_spec)?;

                            to_spec
                                .components
                                .schemas
                                .insert(component_name, ss.clone());
                        }
                    }
                }
            } else if component_ref.file_path.is_none() {
                let hash_key = format!("{current_file_path}#{component_name}");
                if self
                    .combined_schemas
                    .read()
                    .map_err(|e| format!("Lock poisoned while reading combined_schemas: {e}"))?
                    .contains_key(&hash_key)
                {
                    self.logger.info(format!(
                        "Schema `{component_name}` already loaded from file `{current_file_path}`"
                    ));
                } else {
                    self.combined_schemas
                        .write()
                        .map_err(|e| format!("Lock poisoned while writing combined_schemas: {e}"))?
                        .insert(hash_key, true);

                    let include_spec = self.loader.load(current_file_path)?;

                    let component: Option<&Schema> =
                        include_spec.components.schemas.get(component_name.as_str());

                    match component {
                        None => {
                            return Err(format!(
                                "Schema #/components/schemas/{} is not found",
                                component_name.clone()
                            ));
                        }
                        Some(s) => {
                            let ss: &mut Schema = &mut s.clone();
                            self.collect_schema_properties(ss, current_file_path, to_spec)?;

                            to_spec
                                .components
                                .schemas
                                .insert(component_name, ss.clone());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn collect_ref_to_request_body(
        &self,
        property: &mut RefOrObject<RequestBody>,
        to_spec: &mut SpecificationRoot,
        current_file_path: &str,
    ) -> Result<(), String> {
        if let RefOrObject::Ref(r) = property {
            let component_ref = ComponentRef::try_from(r.reference.clone())?;
            if !component_ref.is_request_body() {
                return Err(format!(
                    "Wrong reference to RequestBody component: `{}`",
                    r.reference.clone()
                ));
            }

            if let Some(file_path) = component_ref.file_path {
                let dir = Path::new(current_file_path)
                    .parent()
                    .ok_or("Can't read parent file path")?;
                let dir = dir
                    .to_str()
                    .ok_or("Can't convert parent file path into String")?;

                let inner_file_path = format!("{}/{}", dir.to_string().clone(), file_path);
                //load from filesystem
                let include_spec = self.loader.load(&inner_file_path)?;

                let component: Option<&RequestBody> = include_spec
                    .components
                    .request_bodies
                    .get(component_ref.component_name.as_str());
                match component {
                    None => {
                        return Err(format!(
                            "Request body #/components/requestBodies/{} is not found",
                            component_ref.component_name.clone()
                        ));
                    }
                    Some(s) => {
                        let b_name = component_ref.component_name;

                        let reference = format!("#/components/requestBodies/{b_name}");
                        r.reference = reference;

                        let b: &mut RequestBody = &mut s.clone();

                        let (res_spec, res_body) =
                            self.collect_request_body(b, &inner_file_path)?;

                        // collect_schema_properties(ss, inner_file_path.into(), to_spec);

                        for (n, s) in res_spec.components.schemas {
                            to_spec.components.schemas.insert(n, s.clone());
                        }

                        to_spec.components.request_bodies.insert(b_name, res_body);
                    }
                }
            }
        }

        Ok(())
    }

    fn collect_ref_to_response(
        &self,
        property: &mut RefOrObject<Response>,
        to_spec: &mut SpecificationRoot,
        current_file_path: &str,
    ) -> Result<(), String> {
        if let RefOrObject::Ref(r) = property {
            let component_ref = ComponentRef::try_from(r.reference.clone())?;
            if !component_ref.is_response() {
                return Err(format!(
                    "Wrong reference to response component: `{}`",
                    r.reference.clone()
                ));
            }

            if let Some(file_path) = component_ref.file_path {
                let dir = Path::new(current_file_path)
                    .parent()
                    .ok_or("Can't read current file path")?;
                let dir = dir
                    .to_str()
                    .ok_or("Can't convert current file path into String")?;

                let inner_file_path = format!("{}/{}", dir.to_string().clone(), file_path);
                //load from filesystem
                let include_spec = self.loader.load(&inner_file_path)?;

                let component: Option<&Response> = include_spec
                    .components
                    .responses
                    .get(component_ref.component_name.as_str());
                match component {
                    None => {
                        return Err(format!(
                            "Response #/components/responses/{} is not found",
                            component_ref.component_name.clone()
                        ));
                    }
                    Some(s) => {
                        let b_name = component_ref.component_name;

                        let reference = format!("#/components/responses/{b_name}");
                        r.reference = reference;

                        let resp: &mut Response = &mut s.clone();

                        let (res_spec, res_resp) = self.collect_response(resp, &inner_file_path)?;

                        for (n, s) in res_spec.components.schemas {
                            to_spec.components.schemas.insert(n, s.clone());
                        }

                        to_spec.components.responses.insert(b_name, res_resp);
                    }
                }
            }
        }

        Ok(())
    }

    fn collect_properties(
        &self,
        properties: &mut BTreeMap<String, RefOrObject<Schema>>,
        to_spec: &mut SpecificationRoot,
        current_file_path: &str,
    ) -> Result<(), String> {
        for (_name, property) in properties.iter_mut() {
            self.collect_ref_to_schema(property, to_spec, current_file_path)?;
        }

        Ok(())
    }

    pub fn combine(&self, file_path: &str) -> Result<SpecificationRoot, String> {
        let mut spec = self.loader.load(file_path)?;
        let mut res_spec = SpecificationRoot::new_empty();

        res_spec.openapi.clone_from(&spec.openapi);
        res_spec.external_docs.clone_from(&spec.external_docs);
        res_spec.info.clone_from(&spec.info);
        res_spec.security.clone_from(&spec.security);
        res_spec.tags.clone_from(&spec.tags);
        res_spec.servers.clone_from(&spec.servers);

        for (body_name, body) in &mut spec.components.request_bodies {
            let (body_spec, updated_body) = self.collect_request_body(body, file_path)?;

            for (n, s) in body_spec.components.schemas {
                res_spec.components.schemas.insert(n, s.clone());
            }

            res_spec
                .components
                .request_bodies
                .insert(body_name.clone(), updated_body);
        }

        for (response_name, response) in &mut spec.components.responses {
            let (body_spec, updated_response) = self.collect_response(response, file_path)?;

            for (n, s) in body_spec.components.schemas {
                res_spec.components.schemas.insert(n, s.clone());
            }

            res_spec
                .components
                .responses
                .insert(response_name.clone(), updated_response);
        }

        for (schema_name, schema) in &mut spec.components.schemas {
            let (schemas_spec, updated_schema) = self.collect_schema(schema, file_path)?;

            for (n, s) in schemas_spec.components.schemas {
                res_spec.components.schemas.insert(n, s.clone());
            }

            res_spec
                .components
                .schemas
                .insert(schema_name.clone(), updated_schema);
        }

        for (uri, path) in &mut spec.paths {
            for operation in path.operations.values_mut() {
                if let Some(ref_or_schema) = &mut operation.request_body {
                    self.collect_ref_to_request_body(ref_or_schema, &mut res_spec, file_path)?;
                }

                for ref_or_response in operation.responses.values_mut() {
                    self.collect_ref_to_response(ref_or_response, &mut res_spec, file_path)?;
                }
            }

            res_spec.paths.insert(uri.clone(), path.clone());
        }

        res_spec.components.security_schemes = spec.components.security_schemes.clone();

        Ok(res_spec)
    }
}
