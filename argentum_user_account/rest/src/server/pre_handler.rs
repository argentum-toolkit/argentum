use crate::dto::request::AnonymousRequestsRestoreTokenRequest;
use crate::dto::request::AnonymousWithTokenChangesPasswordRequest;
use crate::dto::request::UserLoginsWithPasswordRequest;
use crate::dto::request::UserRegistersWithPasswordRequest;

use crate::server::handler::AnonymousRegistersTrait;
use crate::server::handler::AnonymousRequestsRestoreTokenTrait;
use crate::server::handler::AnonymousWithTokenChangesPasswordTrait;
use crate::server::handler::UserLoginsWithPasswordTrait;
use crate::server::handler::UserRegistersWithPasswordTrait;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, RequestTrait};
use argentum_rest_infrastructure::service::{BearerAuthenticator, RequestTransformer};
use std::collections::HashMap;
use std::sync::Arc;

pub struct UserAccountPreHandler {
    request_transformer: Arc<RequestTransformer>,
    bearer_auth: Arc<BearerAuthenticator>,
    anonymous_registers: Arc<dyn AnonymousRegistersTrait>,
    anonymous_requests_restore_token: Arc<dyn AnonymousRequestsRestoreTokenTrait>,
    anonymous_with_token_changes_password: Arc<dyn AnonymousWithTokenChangesPasswordTrait>,
    user_logins_with_password: Arc<dyn UserLoginsWithPasswordTrait>,
    user_registers_with_password: Arc<dyn UserRegistersWithPasswordTrait>,
}

impl UserAccountPreHandler {
    pub fn new(
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator>,
        anonymous_registers: Arc<dyn AnonymousRegistersTrait>,
        anonymous_requests_restore_token: Arc<dyn AnonymousRequestsRestoreTokenTrait>,
        anonymous_with_token_changes_password: Arc<dyn AnonymousWithTokenChangesPasswordTrait>,
        user_logins_with_password: Arc<dyn UserLoginsWithPasswordTrait>,
        user_registers_with_password: Arc<dyn UserRegistersWithPasswordTrait>,
    ) -> Self {
        UserAccountPreHandler {
            request_transformer,
            bearer_auth,
            anonymous_registers,
            anonymous_requests_restore_token,
            anonymous_with_token_changes_password,
            user_logins_with_password,
            user_registers_with_password,
        }
    }

    pub async fn anonymous_registers(&self) -> Result<HttpResponse, HttpError> {
        let r = self.anonymous_registers.handle()?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }

    pub async fn anonymous_requests_restore_token(
        &self,
        request: impl RequestTrait,
    ) -> Result<HttpResponse, HttpError> {
        let raw_params = HashMap::from([]);
        let req: AnonymousRequestsRestoreTokenRequest = self
            .request_transformer
            .transform(request, raw_params)
            .await?;

        let user = self.bearer_auth.auth(&req.params.headers)?;

        let r = self.anonymous_requests_restore_token.handle(req, user)?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }

    pub async fn anonymous_with_token_changes_password(
        &self,
        request: impl RequestTrait,
    ) -> Result<HttpResponse, HttpError> {
        let raw_params = HashMap::from([]);
        let req: AnonymousWithTokenChangesPasswordRequest = self
            .request_transformer
            .transform(request, raw_params)
            .await?;

        let user = self.bearer_auth.auth(&req.params.headers)?;

        let r = self
            .anonymous_with_token_changes_password
            .handle(req, user)?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }

    pub async fn user_logins_with_password(
        &self,
        request: impl RequestTrait,
    ) -> Result<HttpResponse, HttpError> {
        let raw_params = HashMap::from([]);
        let req: UserLoginsWithPasswordRequest = self
            .request_transformer
            .transform(request, raw_params)
            .await?;

        let user = self.bearer_auth.auth(&req.params.headers)?;

        let r = self.user_logins_with_password.handle(req, user)?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }

    pub async fn user_registers_with_password(
        &self,
        request: impl RequestTrait,
    ) -> Result<HttpResponse, HttpError> {
        let raw_params = HashMap::from([]);
        let req: UserRegistersWithPasswordRequest = self
            .request_transformer
            .transform(request, raw_params)
            .await?;

        let user = self.bearer_auth.auth(&req.params.headers)?;

        let r = self.user_registers_with_password.handle(req, user)?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }
}
