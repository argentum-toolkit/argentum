use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::cli_params::ApiDocParams;
use crate::generator::api_doc::{
    OpenApiGenerator, PathsGenerator, RequestGenerator, ResponseGenerator, TypeGenerator,
};

pub(crate) struct ApiDocGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    open_api: Arc<OpenApiGenerator<L>>,
    paths: Arc<PathsGenerator<L>>,
    request: Arc<RequestGenerator<L>>,
    response: Arc<ResponseGenerator<L>>,
    r#type: Arc<TypeGenerator<L>>,
}

impl<L> ApiDocGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(
        logger: Arc<L>,
        open_api: Arc<OpenApiGenerator<L>>,
        paths: Arc<PathsGenerator<L>>,
        request: Arc<RequestGenerator<L>>,
        response: Arc<ResponseGenerator<L>>,
        r#type: Arc<TypeGenerator<L>>,
    ) -> Self {
        Self {
            logger,
            open_api,
            paths,
            request,
            response,
            r#type,
        }
    }

    pub fn generate(&self, params: &ApiDocParams) -> Result<(), String> {
        self.open_api.generate(params)?;
        self.paths.generate(params)?;
        self.request.generate(params)?;
        self.response.generate(params)?;
        self.r#type.generate(params)?;

        self.logger.info("Api-doc folder is generated.");

        Ok(())
    }
}
