use crate::service::{
    BearerAuthenticator, ErrorHandler, ErrorPreHandler, HeaderParamsExtractor, PathParamsExtractor,
    QueryParamsExtractor, RequestTransformer, ResponseToJsonTransformer, SchemaExtractor,
    ValidationErrorTransformer,
};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::{data_type::id::Id, error::DiError};
use argentum_user_business::di::UserBusinessDiBuilder;
use std::sync::{Arc, Mutex};

pub struct RestDi<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub bearer_authenticator: Arc<BearerAuthenticator<I>>,
    pub request_transformer: Arc<RequestTransformer>,
    pub response_transformer: Arc<ResponseToJsonTransformer>,
    pub error_pre_handler: Arc<ErrorPreHandler>,
    pub error_handler: Arc<ErrorHandler<L>>,
}

pub struct RestDiBuilder<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    user_business_builder: Arc<Mutex<UserBusinessDiBuilder<I, L>>>,
    logger: Arc<L>,

    built_di: Mutex<Option<Arc<RestDi<I, L>>>>,
}

impl<I, L> RestDiBuilder<I, L>
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    pub fn new(
        user_business_builder: Arc<Mutex<UserBusinessDiBuilder<I, L>>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            user_business_builder,
            logger,

            built_di: Mutex::new(None),
        }
    }

    pub fn build(&self) -> Result<Arc<RestDi<I, L>>, DiError> {
        if let Some(di) = self.built_di.lock()?.clone() {
            return Ok(di.clone());
        }

        let validation_error_transformer = Arc::new(ValidationErrorTransformer::new());
        let schema_extractor = Arc::new(SchemaExtractor::new());
        let header_params_extractor = Arc::new(HeaderParamsExtractor::new(
            validation_error_transformer.clone(),
        ));
        let path_params_extractor = Arc::new(PathParamsExtractor::new(
            validation_error_transformer.clone(),
        ));
        let query_params_extractor =
            Arc::new(QueryParamsExtractor::new(validation_error_transformer));

        let request_transformer = Arc::new(RequestTransformer::new(
            schema_extractor,
            header_params_extractor,
            path_params_extractor,
            query_params_extractor,
        ));
        let response_transformer = Arc::new(ResponseToJsonTransformer::new());
        let error_pre_handler = Arc::new(ErrorPreHandler::new());
        let error_handler = Arc::new(ErrorHandler::new(self.logger.clone()));

        let user_di = self.user_business_builder.lock()?.build()?;

        let bearer_authenticator = Arc::new(BearerAuthenticator::new(
            user_di.user_authenticates_with_token_uc.clone(),
        ));

        let di = Arc::new(RestDi {
            bearer_authenticator,
            request_transformer,
            response_transformer,
            error_pre_handler,
            error_handler,
        });

        *self.built_di.lock()? = Some(di.clone());

        Ok(di)
    }
}
