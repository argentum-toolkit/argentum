use crate::service::{
    BearerAuthenticator, ErrorHandler, ErrorPreHandler, HeaderParamsExtractor, PathParamsExtractor,
    RequestTransformer, ResponseToJsonTransformer, SchemaExtractor, ValidationErrorTransformer,
};
use argentum_log_business::LoggerTrait;
use argentum_user_account_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use std::sync::Arc;

pub struct RestDiC {
    pub request_transformer: Arc<RequestTransformer>,
    pub response_transformer: Arc<ResponseToJsonTransformer>,
    pub error_pre_handler: Arc<ErrorPreHandler>,
    pub error_handler: Arc<ErrorHandler>,
    pub bearer_authenticator: Arc<BearerAuthenticator>,
}

impl RestDiC {
    pub fn new(
        logger: Arc<dyn LoggerTrait>,
        user_authenticates_with_token_uc: Arc<UserAuthenticatesWithTokenUc>,
    ) -> Self {
        let validation_error_transformer = Arc::new(ValidationErrorTransformer::new());
        let schema_extractor = Arc::new(SchemaExtractor::new());
        let header_params_extractor = Arc::new(HeaderParamsExtractor::new(
            validation_error_transformer.clone(),
        ));

        let path_params_extractor =
            Arc::new(PathParamsExtractor::new(validation_error_transformer));
        let request_transformer = Arc::new(RequestTransformer::new(
            schema_extractor,
            header_params_extractor,
            path_params_extractor,
        ));
        let response_transformer = Arc::new(ResponseToJsonTransformer::new());
        let error_pre_handler = Arc::new(ErrorPreHandler::new());
        let error_handler = Arc::new(ErrorHandler::new(logger));

        let bearer_authenticator =
            Arc::new(BearerAuthenticator::new(user_authenticates_with_token_uc));

        RestDiC {
            request_transformer,
            response_transformer,
            error_pre_handler,
            error_handler,
            bearer_authenticator,
        }
    }
}
