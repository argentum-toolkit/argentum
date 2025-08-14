use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    ComponentRef, Method, Parameter, RefOrObject, SecurityRequirementObject, SpecificationRoot,
};
use reqwest::StatusCode;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::Arc;

const MOD_PATH: &str = "/src/client/mod.rs";
const MOD_TEMPLATE: &str = "client/mod";

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ContentData {
    pub content_type: String,
    pub schema_name: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ResponseData {
    pub status_name: String,
    pub status_code: String,
    pub response_name: String,
    pub content: Vec<ContentData>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct OperationData {
    operation_id: String,
    need_body: bool,
    method: String,
    pub security: Option<Vec<SecurityRequirementObject>>,
    pub responses: Vec<ResponseData>,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct PathData {
    pub url: String,
    pub operations: Vec<OperationData>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    pub paths: Vec<PathData>,
    pub security_enabled: bool,
    pub use_schemas: Vec<String>,
    pub use_responses: Vec<String>,
}

fn to_enum_name(code: &str) -> Result<String, String> {
    let m: BTreeMap<&str, &str> = BTreeMap::from([
        ("100", "CONTINUE"),
        ("101", "SWITCHING_PROTOCOLS"),
        ("102", "PROCESSING"),
        ("200", "OK"),
        ("201", "CREATED"),
        ("202", "ACCEPTED"),
        ("203", "NON_AUTHORITATIVE_INFORMATION"),
        ("204", "NO_CONTENT"),
        ("205", "RESET_CONTENT"),
        ("206", "PARTIAL_CONTENT"),
        ("207", "MULTI_STATUS"),
        ("208", "ALREADY_REPORTED"),
        ("226", "IM_USED"),
        ("300", "MULTIif methPLE_CHOICES"),
        ("301", "MOVED_PERMANENTLY"),
        ("302", "FOUND"),
        ("303", "SEE_OTHER"),
        ("304", "NOT_MODIFIED"),
        ("305", "USE_PROXY"),
        ("307", "TEMPORARY_REDIRECT"),
        ("308", "PERMANENT_REDIRECT"),
        ("400", "BAD_REQUEST"),
        ("401", "UNAUTHORIZED"),
        ("402", "PAYMENT_REQUIRED"),
        ("403", "FORBIDDEN"),
        ("404", "NOT_FOUND"),
        ("405", "METHOD_NOT_ALLOWED"),
        ("406", "NOT_ACCEPTABLE"),
        ("407", "PROXY_AUTHENTICATION_REQUIRED"),
        ("408", "REQUEST_TIMEOUT"),
        ("409", "CONFLICT"),
        ("410", "GONE"),
        ("411", "LENGTH_REQUIRED"),
        ("412", "PRECONDITION_FAILED"),
        ("413", "PAYLOAD_TOO_LARGE"),
        ("414", "URI_TOO_LONG"),
        ("415", "UNSUPPORTED_MEDIA_TYPE"),
        ("416", "RANGE_NOT_SATISFIABLE"),
        ("417", "EXPECTATION_FAILED"),
        ("418", "IM_A_TEAPOT"),
        ("421", "MISDIRECTED_REQUEST"),
        ("422", "UNPROCESSABLE_ENTITY"),
        ("423", "LOCKED"),
        ("424", "FAILED_DEPENDENCY"),
        ("426", "UPGRADE_REQUIRED"),
        ("428", "PRECONDITION_REQUIRED"),
        ("429", "TOO_MANY_REQUESTS"),
        ("431", "REQUEST_HEADER_FIELDS_TOO_LARGE"),
        ("451", "UNAVAILABLE_FOR_LEGAL_REASONS"),
        ("500", "INTERNAL_SERVER_ERROR"),
        ("501", "NOT_IMPLEMENTED"),
        ("502", "BAD_GATEWAY"),
        ("503", "SERVICE_UNAVAILABLE"),
        ("504", "GATEWAY_TIMEOUT"),
        ("505", "HTTP_VERSION_NOT_SUPPORTED"),
        ("506", "VARIANT_ALSO_NEGOTIATES"),
        ("507", "INSUFFICIENT_STORAGE"),
        ("508", "LOOP_DETECTED"),
        ("510", "NOT_EXTENDED"),
        ("511", "NETWORK_AUTHENTICATION_REQUIRED"),
    ]);

    m.get(code)
        .map(|c| c.to_string())
        .ok_or(format!("Unknown code `{code}`").into())
}

pub struct ClientGenerator {
    renderer: Arc<Renderer>,
}

impl ClientGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    pub fn generate(&self, base_output_path: &str, spec: &SpecificationRoot) -> Result<(), String> {
        let mut paths_data: Vec<PathData> = vec![];
        let mut security_enabled = false;
        let mut use_schemas: Vec<String> = vec![];
        let mut use_responses: Vec<String> = vec![];

        for (url, path) in spec.clone().paths {
            let mut operations: Vec<OperationData> = vec![];
            for (method, operation) in path.operations.clone() {
                let uri_parameters = path.parameters.clone();
                let mut parameters: Vec<Parameter> = vec![];

                if let Some(params) = uri_parameters {
                    for param in params {
                        parameters.push(param.clone())
                    }
                };

                if let Some(params) = &operation.parameters {
                    for param in params {
                        parameters.push(param.clone())
                    }
                }

                if operation.security.is_some() {
                    security_enabled = true;
                }

                let mut response_data: Vec<ResponseData> = vec![];

                for (code, ref_or_obj) in &operation.responses {
                    let status_name = match StatusCode::from_str(&code.to_string()) {
                        Ok(c) => to_enum_name(c.as_str())?,
                        Err(e) => return Err(format!("Can't parse status code: {e:?}").into()),
                    };

                    let mut content_data: Vec<ContentData> = vec![];
                    let response_name: String;
                    match ref_or_obj {
                        RefOrObject::Ref(r) => {
                            let component_ref = ComponentRef::try_from(r.reference.clone())?;
                            if !component_ref.is_response() {
                                return Err(format!(
                                    "Wrong reference to response component: `{}`",
                                    r.reference.clone()
                                )
                                .into());
                            }

                            response_name =
                                self.escape_response_name(component_ref.component_name.clone());

                            use_responses.push(response_name.clone());

                            let response = spec
                                .components
                                .responses
                                .get(&component_ref.component_name)
                                .ok_or("Response component `response_name` not found")?;

                            for (content_type, media) in &response.content {
                                let schema_name = match &media.schema {
                                    RefOrObject::Ref(schema_ref) => {
                                        let component_ref =
                                            ComponentRef::try_from(schema_ref.reference.clone())?;
                                        if !component_ref.is_schema() {
                                            return Err(format!(
                                                "Wrong reference to schema component: `{}`",
                                                r.reference.clone()
                                            )
                                            .into());
                                        }

                                        component_ref.component_name
                                    }
                                    RefOrObject::Object(_) => {
                                        return Err("We don't support inline objects yet. Only Refs are allowed. Skipping...".into());
                                    }
                                };

                                use_schemas.push(schema_name.to_string());

                                content_data.push(ContentData {
                                    content_type: content_type.clone(),
                                    schema_name,
                                })
                            }
                        }
                        RefOrObject::Object(_) => {
                            return Err("We don't support inline objects yet. Only Refs are allowed. Skipping...".into());
                        }
                    };

                    response_data.push(ResponseData {
                        status_name,
                        status_code: code.to_string(),
                        content: content_data,
                        response_name,
                    });
                }

                let need_body = matches!(method, Method::Post);

                operations.push(OperationData {
                    method: method.to_string(),
                    need_body,
                    security: operation.security,
                    operation_id: operation.operation_id,
                    responses: response_data,
                    parameters,
                });
            }

            let item = PathData { url, operations };

            paths_data.push(item);
        }

        use_schemas.sort();
        use_schemas.dedup();
        use_responses.sort();
        use_responses.dedup();

        let data = Data {
            paths: paths_data,
            security_enabled,
            use_schemas,
            use_responses,
        };

        self.renderer
            .render(base_output_path, MOD_TEMPLATE, data, MOD_PATH)?;

        Ok(())
    }

    fn escape_response_name(&self, name: String) -> String {
        if name[0..1].parse::<u8>().is_ok() {
            "Status".to_owned() + &name
        } else {
            name
        }
    }
}
