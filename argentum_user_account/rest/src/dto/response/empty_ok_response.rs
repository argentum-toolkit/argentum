use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;
use argentum_rest_infrastructure::data_type::SerializableBody;

use crate::dto::schema::EmptyResponse;

#[derive(Clone)]
pub struct ApplicationJson(EmptyResponse);

impl ContentTypeResponseTrait for ApplicationJson {
    fn content_type(&self) -> String {
        "application/json".to_string()
    }

    fn body(&self) -> Box<dyn SerializableBody> {
        Box::new(self.0.clone())
    }
}

pub enum EmptyOkResponse {
    ApplicationJson(ApplicationJson),
}

impl EmptyOkResponse {
    pub fn new_application_json(r: EmptyResponse) -> Self {
        Self::ApplicationJson(ApplicationJson(r))
    }

    pub fn to_content_type_response_trait(&self) -> Box<dyn ContentTypeResponseTrait> {
        match self {
            Self::ApplicationJson(r) => Box::new(r.clone()),
        }
    }
}
