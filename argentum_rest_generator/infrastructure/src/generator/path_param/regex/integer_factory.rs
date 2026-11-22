use argentum_openapi_infrastructure::data_type::Schema;

pub struct IntegerFactory {}

impl IntegerFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create(&self, _schema: Schema, name: String) -> String {
        format!("(?<{}>-?\\d+)", name)
    }
}
