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

impl TryFrom<&str> for ComponentType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "schemas" => Ok(Self::Schema),
            "requestBodies" => Ok(Self::RequestBody),
            "responses" => Ok(Self::Response),
            t => Err(format!("wrong schema type `{t}`")),
        }
    }
}
