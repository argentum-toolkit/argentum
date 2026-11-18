use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;
use argentum_rest_infrastructure::data_type::http_response::NoContent;

#[derive(Clone)]
pub enum Status204Response {
    NoContent(NoContent),
}

impl Status204Response {
    pub fn new_no_content() -> Self {
        Self::NoContent(NoContent {})
    }

    pub fn to_content_type_response_trait(&self) -> Box<dyn ContentTypeResponseTrait> {
        NoContent::new_boxed()
    }
}
