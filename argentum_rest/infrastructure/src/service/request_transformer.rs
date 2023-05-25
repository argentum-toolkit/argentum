use crate::data_type::error::HttpError::BadRequest;
use crate::data_type::error::{BadRequestError, HttpError};
use crate::data_type::{HttpParams, HttpRequest, RequestTrait};
use crate::service::{PathParamsExtractor, RawPathParams, SchemaExtractor};
use argentum_standard_business::invariant_violation::Violations;
use std::sync::Arc;

pub struct RequestTransformer {
    schema_extractor: Arc<SchemaExtractor>,
    path_params_extractor: Arc<PathParamsExtractor>,
}

impl RequestTransformer {
    pub fn new(
        schema_extractor: Arc<SchemaExtractor>,
        path_params_extractor: Arc<PathParamsExtractor>,
    ) -> Self {
        Self {
            schema_extractor,
            path_params_extractor,
        }
    }

    pub async fn transform<R>(
        &self,
        request: impl RequestTrait,
        raw_path_params: RawPathParams,
    ) -> Result<R, HttpError>
    where
        R: HttpRequest,
    {
        //TODO: make pretty errors
        let body_res: Result<R::Body, Violations> = self.schema_extractor.extract(request).await;

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

        if body.is_some() && path_params.is_some() {
            Ok(R::new(body.unwrap(), R::Params::new(path_params.unwrap())))
        } else {
            Err(BadRequest(BadRequestError::new(
                body_violations,
                path_violations,
            )))
        }
    }
}
