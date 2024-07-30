use crate::generator::path_param::regex::integer_factory::IntegerFactory;
use crate::generator::path_param::regex::string_factory::StringFactory;
use argentum_openapi_infrastructure::data_type::{Parameter, RefOrObject, SchemaType};
use std::sync::Arc;

pub struct RegexFactory {
    string_factory: Arc<StringFactory>,
    integer_factory: Arc<IntegerFactory>,
}

impl RegexFactory {
    pub fn new(string_factory: Arc<StringFactory>, integer_factory: Arc<IntegerFactory>) -> Self {
        Self {
            string_factory,
            integer_factory,
        }
    }

    pub fn create(&self, param: &Parameter) -> String {
        match param.schema.clone() {
            RefOrObject::Ref(_r) => {
                //TODO: follow the reference and get the schema
                // r.reference.clone();
                format!("(?<{}>\\w+)", param.name)
            }
            RefOrObject::Object(s) => match s.schema_type {
                Some(SchemaType::String) => self.string_factory.create(s, param.name.clone()),
                Some(SchemaType::Integer) => self.integer_factory.create(s, param.name.clone()),
                _ => {
                    format!("(?<{}>\\w+)", param.name)
                }
            },
        }
    }
}
