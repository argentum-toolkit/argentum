use crate::service::{
    BearerAuthenticator, ErrorHandler, ErrorPreHandler, HeaderParamsExtractor, PathParamsExtractor,
    QueryParamsExtractor, RequestTransformer, ResponseToJsonTransformer, SchemaExtractor,
    ValidationErrorTransformer,
};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::{data_type::id::Id, di_container::Container};
use argentum_user_business::use_case::member_authenticates_with_token::MemberAuthenticatesWithTokenUc;
use std::sync::Arc;

pub fn rest_infrastructure_provide<I, L>(container: &mut Container)
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    container.provide_factory::<ValidationErrorTransformer, _>(|_c| {
        Ok(Arc::new(ValidationErrorTransformer::new()))
    });

    container.provide_factory::<SchemaExtractor, _>(|_c| Ok(Arc::new(SchemaExtractor::new())));

    container.provide_factory::<HeaderParamsExtractor, _>(|c| {
        let validation_error_transformer = c.resolve::<ValidationErrorTransformer>()?;

        Ok(Arc::new(HeaderParamsExtractor::new(
            validation_error_transformer,
        )))
    });

    container.provide_factory::<PathParamsExtractor, _>(|c| {
        let validation_error_transformer = c.resolve::<ValidationErrorTransformer>()?;

        Ok(Arc::new(PathParamsExtractor::new(
            validation_error_transformer,
        )))
    });

    container.provide_factory::<QueryParamsExtractor, _>(|c| {
        let validation_error_transformer = c.resolve::<ValidationErrorTransformer>()?;

        Ok(Arc::new(QueryParamsExtractor::new(
            validation_error_transformer,
        )))
    });

    container.provide_factory::<RequestTransformer, _>(|c| {
        let schema_extractor = c.resolve::<SchemaExtractor>()?;
        let header_params_extractor = c.resolve::<HeaderParamsExtractor>()?;
        let path_params_extractor = c.resolve::<PathParamsExtractor>()?;
        let query_params_extractor = c.resolve::<QueryParamsExtractor>()?;

        Ok(Arc::new(RequestTransformer::new(
            schema_extractor,
            header_params_extractor,
            path_params_extractor,
            query_params_extractor,
        )))
    });

    container.provide_factory::<ResponseToJsonTransformer, _>(|_c| {
        Ok(Arc::new(ResponseToJsonTransformer::new()))
    });

    container.provide_factory::<ErrorPreHandler, _>(|_c| Ok(Arc::new(ErrorPreHandler::new())));

    container.provide_factory::<ErrorHandler<L>, _>(|c| {
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(ErrorHandler::new(logger)))
    });

    container.provide_factory::<BearerAuthenticator<I>, _>(|c| {
        let user_authenticates_with_token_uc = c.resolve::<MemberAuthenticatesWithTokenUc<I>>()?;

        Ok(Arc::new(BearerAuthenticator::new(
            user_authenticates_with_token_uc,
        )))
    });
}
