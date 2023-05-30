use crate::api::server::handler::{AnonymousRegistersTrait, UserRegistersWithPasswordTrait};
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, RequestTrait};
use argentum_rest_infrastructure::service::RequestTransformer;
use std::collections::HashMap;
use std::sync::Arc;

pub struct TodoPreHandler {
    request_transformer: Arc<RequestTransformer>,
    anonymous_registers: Arc<dyn AnonymousRegistersTrait>,
    user_registers_with_password: Arc<dyn UserRegistersWithPasswordTrait>,
}

impl TodoPreHandler {
    pub fn new(
        request_transformer: Arc<RequestTransformer>,
        anonymous_registers: Arc<dyn AnonymousRegistersTrait>,
        user_registers_with_password: Arc<dyn UserRegistersWithPasswordTrait>,
    ) -> Self {
        TodoPreHandler {
            request_transformer,
            anonymous_registers,
            user_registers_with_password,
        }
    }

    pub async fn handle_anonymous_registers(&self) -> Result<HttpResponse, HttpError> {
        self.anonymous_registers.handle()
    }

    pub async fn handle_user_registers_with_password(
        &self,
        request: impl RequestTrait,
    ) -> Result<HttpResponse, HttpError> {
        let raw_params = HashMap::from([]);
        let req = self
            .request_transformer
            .transform(request, raw_params)
            .await?;

        self.user_registers_with_password.handle(req)
    }
}
