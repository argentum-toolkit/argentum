use crate::api::dto::request::{UserLoginsWithPasswordRequest, UserRegistersWithPasswordRequest};
use crate::api::server::handler::{
    AnonymousRegistersTrait, UserLoginsWithPasswordTrait, UserRegistersWithPasswordTrait,
};
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, RequestTrait};
use argentum_rest_infrastructure::service::{BearerAuthenticator, RequestTransformer};
use std::collections::HashMap;
use std::sync::Arc;

pub struct UserAccountPreHandler {
    request_transformer: Arc<RequestTransformer>,
    bearer_auth: Arc<BearerAuthenticator>,
    anonymous_registers: Arc<dyn AnonymousRegistersTrait>,
    user_logins_with_password: Arc<dyn UserLoginsWithPasswordTrait>,
    user_registers_with_password: Arc<dyn UserRegistersWithPasswordTrait>,
}

impl UserAccountPreHandler {
    pub fn new(
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator>,
        anonymous_registers: Arc<dyn AnonymousRegistersTrait>,
        user_logins_with_password: Arc<dyn UserLoginsWithPasswordTrait>,
        user_registers_with_password: Arc<dyn UserRegistersWithPasswordTrait>,
    ) -> Self {
        UserAccountPreHandler {
            request_transformer,
            bearer_auth,
            anonymous_registers,
            user_logins_with_password,
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
        let req: UserRegistersWithPasswordRequest = self
            .request_transformer
            .transform(request, raw_params)
            .await?;

        let user = self.bearer_auth.auth(&req.params.headers)?;

        self.user_registers_with_password.handle(req, user)
    }

    pub async fn handle_user_logins_with_password(
        &self,
        request: impl RequestTrait,
    ) -> Result<HttpResponse, HttpError> {
        let raw_params = HashMap::from([]);
        let req: UserLoginsWithPasswordRequest = self
            .request_transformer
            .transform(request, raw_params)
            .await?;

        let user = self.bearer_auth.auth(&req.params.headers)?;

        self.user_logins_with_password.handle(req, user)
    }
}
