use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;
use argentum_rest_infrastructure::data_type::SerializableBody;

use crate::dto::schema::ProblemDetail;

#[derive(Clone)]
pub struct ApplicationProblemJson(ProblemDetail);

impl ContentTypeResponseTrait for ApplicationProblemJson {
    fn content_type(&self) -> Option<String> {
        Some("application/problem+json".to_string())
    }

    fn body(&self) -> Box<dyn SerializableBody> {
        Box::new(self.0.clone())
    }
}

pub enum Status422Response {
    ApplicationProblemJson(ApplicationProblemJson),
}

impl Status422Response {
    pub fn new_application_problem_json(r: ProblemDetail) -> Self {
        Self::ApplicationProblemJson(ApplicationProblemJson(r))
    }

    pub fn to_content_type_response_trait(&self) -> Box<dyn ContentTypeResponseTrait> {
        match self {
            Self::ApplicationProblemJson(r) => Box::new(r.clone()),
        }
    }
}
