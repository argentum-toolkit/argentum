use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;

use crate::dto::schema::User;

#[derive(Clone)]
pub struct ApplicationJson(pub User);

impl ContentTypeResponseTrait for ApplicationJson {
    fn content_type(&self) -> Option<String> {
        Some("application/json".to_string())
    }

    fn body(&self) -> Box<dyn SerializableBody> {
        Box::new(self.0.clone())
    }
}

#[derive(Clone)]
pub enum GetUserOkResponse {
    ApplicationJson(ApplicationJson),
}

impl GetUserOkResponse {
    pub fn new_application_json(r: User) -> Self {
        Self::ApplicationJson(ApplicationJson(r))
    }

    pub fn to_content_type_response_trait(&self) -> Box<dyn ContentTypeResponseTrait> {
        match self {
            Self::ApplicationJson(r) => Box::new(r.clone()),
        }
    }
}
