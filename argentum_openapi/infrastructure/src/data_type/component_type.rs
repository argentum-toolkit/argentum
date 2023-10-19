#[derive(PartialEq)]
pub enum ComponentType {
    Schema,
    RequestBody,
    Response,
    Parameter,
    Example,
    Header,
    SecuritySchema,
    Link,
    Callback,
    PathItem,
}

impl From<&str> for ComponentType {
    fn from(value: &str) -> Self {
        match value {
            "schemas" => Self::Schema,
            "requestBodies" => Self::RequestBody,
            _ => panic!("wrong schema type")
        }
    }
}
