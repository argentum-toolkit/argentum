use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use argentum_openapi_infrastructure::data_type::{ComponentRef, RefOrObject, RequestBody, Response, Schema, SchemaType, SpecificationRoot};

pub(crate) mod generator;
pub(crate) mod template;


fn load_openapi(file_path: String) -> (SpecificationRoot, PathBuf) {
    let path = PathBuf::from(file_path);

    let f = fs::File::open(path.clone()).expect("LogRocket: Should have been able to read the file");

    let spec: SpecificationRoot = serde_yaml::from_reader(f).expect("Could not read values.");

    (spec, path)
}

fn collect_request_boby(body: &mut RequestBody, current_file_path: PathBuf) -> (SpecificationRoot, RequestBody) {
    let mut to_spec = SpecificationRoot::new_empty();

    for (_content_type, mut media_type) in &mut body.content {
        let ref_or_schema = &mut media_type.schema;

        collect_ref_to_schema(ref_or_schema, &mut to_spec, current_file_path.clone());
    }

    (to_spec, body.clone())
}

fn collect_response(response: &mut Response, current_file_path: PathBuf) -> (SpecificationRoot, Response) {
    let mut to_spec = SpecificationRoot::new_empty();

    for (_content_type, mut media_type) in &mut response.content {
        let ref_or_schema = &mut media_type.schema;

        collect_ref_to_schema(ref_or_schema, &mut to_spec, current_file_path.clone());
    }

    (to_spec, response.clone())
}


fn combine(spec: &mut SpecificationRoot, current_file_path: PathBuf) -> SpecificationRoot {
    let mut res_spec = SpecificationRoot::new_empty();

    for (body_name, mut body) in &mut spec.components.request_bodies {
        let (body_spec, updated_body) = collect_request_boby(body, current_file_path.clone());

        for (n, s) in body_spec.components.schemas {
            res_spec.components.schemas.insert(n, s.clone());
        }

        res_spec.components.request_bodies.insert(body_name.clone(), updated_body);
    }

    for (response_name, mut response) in &mut spec.components.responses {
        let (body_spec, updated_response) = collect_response(response, current_file_path.clone());

        for (n, s) in body_spec.components.schemas {
            res_spec.components.schemas.insert(n, s.clone());
        }

        res_spec.components.responses.insert(response_name.clone(), updated_response);
    }

    for (schema_name, mut schema) in &mut spec.components.schemas {
        let (schemas_spec, updated_schema) = collect_schema(schema, current_file_path.clone());

        for (n, s) in schemas_spec.components.schemas {
            res_spec.components.schemas.insert(n, s.clone());
        }

        res_spec.components.schemas.insert(schema_name.clone(), updated_schema);
    }

    for (uri, mut path) in &mut spec.paths {
        for (method_name, operation) in &mut path.operations {
            if let Some(ref_or_schema) = &mut operation.request_body {
                collect_ref_to_request_body(ref_or_schema, &mut res_spec, current_file_path.clone());
            }

            //TODO: collect responses
        }

        res_spec.paths.insert(uri.clone(), path.clone());
    }

    return res_spec;
}

fn collect_schema(schema: &mut Schema, current_file_path: PathBuf) -> (SpecificationRoot, Schema) {
    let mut spec = SpecificationRoot::new_empty();

    collect_schema_properties(schema, current_file_path, &mut spec);


    (spec, schema.clone())
}

fn collect_schema_properties(schema: &mut Schema, current_file_path: PathBuf, to_spec: &mut SpecificationRoot) {
    match schema.schema_type {
        Some(SchemaType::Array) => {}
        Some(SchemaType::Object) => {
            if let Some(properties) = schema.properties.as_mut() {
                collect_properties(properties, to_spec, current_file_path);

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
fn collect_ref_to_schema(property: &mut RefOrObject<Schema>, to_spec: &mut SpecificationRoot, current_file_path: PathBuf) {
    if let RefOrObject::Ref(r) = property {
        let component_ref = ComponentRef::from(r.reference.clone());
        if !component_ref.is_schema() {
            panic!("Wrong reference to schema component: `{}`", r.reference.clone());
        }

        if let Some(file_path) = component_ref.file_path {
            let dir = current_file_path.parent().unwrap();
            let dir = dir.to_str().unwrap();

            let inner_file_path = format!("{}/{}", dir.to_string().clone(), file_path);
            //load from filesystem
            let (include_spec, _include_spec_file_path) = load_openapi(inner_file_path.clone());

            let mut component: Option<&Schema> = include_spec.components.schemas.get(component_ref.component_name.as_str());
            match component {
                None => {panic!("Schema #/components/schemas/{} is not found", component_ref.component_name.clone())}
                Some(s) => {

                    let c_name = component_ref.component_name;

                    let reference = format!("#/components/schemas/{}", c_name);
                    r.reference = reference;

                    let ss: &mut Schema = &mut s.clone();
                    collect_schema_properties(ss, inner_file_path.into(), to_spec);

                    to_spec.components.schemas.insert(c_name, ss.clone());

                }
            }
        }
    }
}

fn collect_ref_to_request_body(property: &mut RefOrObject<RequestBody>, to_spec: &mut SpecificationRoot, current_file_path: PathBuf) {
    if let RefOrObject::Ref(r) = property {
        let component_ref = ComponentRef::from(r.reference.clone());
        if !component_ref.is_request_body() {
            panic!("Wrong reference to schema component: `{}`", r.reference.clone());
        }

        if let Some(file_path) = component_ref.file_path {
            let dir = current_file_path.parent().unwrap();
            let dir = dir.to_str().unwrap();

            let inner_file_path = format!("{}/{}", dir.to_string().clone(), file_path);
            //load from filesystem
            let (include_spec, _include_spec_file_path) = load_openapi(inner_file_path.clone());

            let mut component: Option<&RequestBody> = include_spec.components.request_bodies.get(component_ref.component_name.as_str());
            match component {
                None => {panic!("Schema #/components/requestBodies/{} is not found", component_ref.component_name.clone())}
                Some(s) => {

                    let b_name = component_ref.component_name;

                    let reference = format!("#/components/requestBodies/{}", b_name);
                    r.reference = reference;

                    let b: &mut RequestBody = &mut s.clone();

                    let (mut res_spec, res_body) = collect_request_boby(b, current_file_path.clone());

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

fn collect_properties(properties: &mut BTreeMap<String, RefOrObject<Schema>>, to_spec: &mut SpecificationRoot, current_file_path: PathBuf) {
    for (_name, mut property) in properties.into_iter() {
        collect_ref_to_schema(property, to_spec, current_file_path.clone());
    }
}

fn main() {
    // let input = "../argentum_user_account/api-doc/response.oas.yaml".to_string();
    // let input = "../argentum_user_account/api-doc/request.oas.yaml".to_string();
    // let input = "../argentum_user_account/api-doc/type.oas.yaml".to_string();
    let input = "../argentum_user_account/api-doc/openapi.yaml".to_string();
    let (mut spec, path) = load_openapi(input);

    let res = combine(&mut spec, path);


    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("test.yml")
        .expect("Couldn't open file");
    serde_yaml::to_writer(f, &res).unwrap();


    println!("{:#?}", res.components.request_bodies);
    println!("{:#?}", res.components.responses);
    println!("{:#?}", res.components.schemas);
}
