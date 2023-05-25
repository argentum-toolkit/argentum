use crate::service::{
    ErrorHandler, ErrorPreHandler, PathParamsExtractor, RequestTransformer,
    ResponseToJsonTransformer, SchemaExtractor, ValidationErrorTransformer,
};
use std::sync::Arc;

pub struct RestDiC {
    pub request_transformer: Arc<RequestTransformer>,
    pub response_transformer: Arc<ResponseToJsonTransformer>,
    pub error_pre_handler: Arc<ErrorPreHandler>,
    pub error_handler: Arc<ErrorHandler>,
}

impl RestDiC {
    pub fn new() -> Self {
        let validation_error_transformer = Arc::new(ValidationErrorTransformer::new());
        let schema_extractor = Arc::new(SchemaExtractor::new(validation_error_transformer.clone()));
        let path_params_extractor =
            Arc::new(PathParamsExtractor::new(validation_error_transformer));
        let request_transformer = Arc::new(RequestTransformer::new(
            schema_extractor,
            path_params_extractor,
        ));
        let response_transformer = Arc::new(ResponseToJsonTransformer::new());
        let error_pre_handler = Arc::new(ErrorPreHandler::new());
        let error_handler = Arc::new(ErrorHandler::new());

        RestDiC {
            request_transformer,
            response_transformer,
            error_pre_handler,
            error_handler,
        }
    }
}
