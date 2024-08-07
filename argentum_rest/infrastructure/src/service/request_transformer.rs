use crate::data_type::error::HttpError::BadRequest;
use crate::data_type::error::{BadRequestError, HttpError};
use crate::data_type::{HttpParams, HttpRequest, RequestTrait};
use crate::service::{
    HeaderParamsExtractor, PathParamsExtractor, QueryParamsExtractor, RawPathParams,
    RawQueryParams, SchemaExtractor,
};
use argentum_standard_business::invariant_violation::{InvariantResult, Violations};
use std::sync::Arc;

pub struct RequestTransformer {
    schema_extractor: Arc<SchemaExtractor>,
    header_params_extractor: Arc<HeaderParamsExtractor>,
    path_params_extractor: Arc<PathParamsExtractor>,
    query_params_extractor: Arc<QueryParamsExtractor>,
}

impl RequestTransformer {
    pub fn new(
        schema_extractor: Arc<SchemaExtractor>,
        header_params_extractor: Arc<HeaderParamsExtractor>,
        path_params_extractor: Arc<PathParamsExtractor>,
        query_params_extractor: Arc<QueryParamsExtractor>,
    ) -> Self {
        Self {
            schema_extractor,
            header_params_extractor,
            path_params_extractor,
            query_params_extractor,
        }
    }

    pub async fn transform<R>(
        &self,
        request: impl RequestTrait,
        raw_path_params: RawPathParams<'_>,
        raw_query_params: RawQueryParams,
    ) -> Result<R, HttpError>
    where
        R: HttpRequest,
    {
        let header_result = self
            .header_params_extractor
            .extract::<<<R as HttpRequest>::Params as HttpParams>::Headers>(
            request.get_headers().clone(),
        );

        let (header_params, header_violations) = match header_result {
            Ok(p) => (Some(p), Violations::new(vec![], None)),
            Err(e) => (None, e),
        };

        //TODO: make pretty errors
        let body_res: InvariantResult<R::Body> = self.schema_extractor.extract(request).await;

        let (body, body_violations) = match body_res {
            Ok(b) => (Some(b), Violations::new(vec![], None)),
            Err(e) => (None, e),
        };

        let path_result =
            self.path_params_extractor
                .extract::<<<R as HttpRequest>::Params as HttpParams>::Path>(raw_path_params);

        let (path_params, path_violations) = match path_result {
            Ok(p) => (Some(p), Violations::new(vec![], None)),
            Err(e) => (None, e),
        };

        let query_result =
            self.query_params_extractor
                .extract::<<<R as HttpRequest>::Params as HttpParams>::Query>(raw_query_params);

        let (query_params, query_violations) = match query_result {
            Ok(p) => (Some(p), Violations::new(vec![], None)),
            Err(e) => (None, e),
        };

        if let (Some(b), Some(pp), Some(hp), Some(qp)) =
            (body, path_params, header_params, query_params)
        {
            Ok(R::new(b, R::Params::new(pp, qp, hp)))
        } else {
            Err(BadRequest(BadRequestError::new(
                body_violations,
                path_violations,
                query_violations,
                header_violations,
            )))
        }
    }
}
