use crate::dto::request::GuestRegistersRequest;
use crate::dto::request::GuestRequestsRestoreTokenRequest;
use crate::dto::request::GuestWithTokenChangesPasswordRequest;
use crate::dto::request::MemberLoginsWithPasswordRequest;
use crate::dto::request::MemberRegistersWithPasswordRequest;
use crate::dto::request::MemberVerifiesEmailRequest;

use crate::server::handler::GuestRegistersTrait;
use crate::server::handler::GuestRequestsRestoreTokenTrait;
use crate::server::handler::GuestWithTokenChangesPasswordTrait;
use crate::server::handler::MemberLoginsWithPasswordTrait;
use crate::server::handler::MemberRegistersWithPasswordTrait;
use crate::server::handler::MemberVerifiesEmailTrait;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, RequestTrait};
use argentum_rest_infrastructure::service::{BearerAuthenticator, RequestTransformer};
use argentum_standard_business::data_type::id::Id;
use std::collections::HashMap;
use std::sync::Arc;

pub struct PreHandler<I>
where
    I: Id,
{
    request_transformer: Arc<RequestTransformer>,
    bearer_auth: Arc<BearerAuthenticator<I>>,
    guest_registers: Arc<dyn GuestRegistersTrait<Id = I>>,
    guest_requests_restore_token: Arc<dyn GuestRequestsRestoreTokenTrait<Id = I>>,
    guest_with_token_changes_password: Arc<dyn GuestWithTokenChangesPasswordTrait<Id = I>>,
    member_logins_with_password: Arc<dyn MemberLoginsWithPasswordTrait<Id = I>>,
    member_registers_with_password: Arc<dyn MemberRegistersWithPasswordTrait<Id = I>>,
    member_verifies_email: Arc<dyn MemberVerifiesEmailTrait<Id = I>>,
}

impl<I> PreHandler<I>
where
    I: Id,
{
    pub fn new(
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator<I>>,
        guest_registers: Arc<dyn GuestRegistersTrait<Id = I>>,
        guest_requests_restore_token: Arc<dyn GuestRequestsRestoreTokenTrait<Id = I>>,
        guest_with_token_changes_password: Arc<dyn GuestWithTokenChangesPasswordTrait<Id = I>>,
        member_logins_with_password: Arc<dyn MemberLoginsWithPasswordTrait<Id = I>>,
        member_registers_with_password: Arc<dyn MemberRegistersWithPasswordTrait<Id = I>>,
        member_verifies_email: Arc<dyn MemberVerifiesEmailTrait<Id = I>>,
    ) -> Self {
        Self {
            request_transformer,
            bearer_auth,
            guest_registers,
            guest_requests_restore_token,
            guest_with_token_changes_password,
            member_logins_with_password,
            member_registers_with_password,
            member_verifies_email,
        }
    }

    pub async fn guest_registers(
        &self,
        request: impl RequestTrait,
        raw_path_params: HashMap<&str, &str>,
    ) -> Result<HttpResponse, HttpError> {
        let raw_query_params = HashMap::from([]);
        let req: GuestRegistersRequest = self
            .request_transformer
            .transform(request, raw_path_params, raw_query_params)
            .await?;
        let r = self.guest_registers.handle(req).await?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }

    pub async fn guest_requests_restore_token(
        &self,
        request: impl RequestTrait,
        raw_path_params: HashMap<&str, &str>,
    ) -> Result<HttpResponse, HttpError> {
        let raw_query_params = HashMap::from([]);
        let req: GuestRequestsRestoreTokenRequest = self
            .request_transformer
            .transform(request, raw_path_params, raw_query_params)
            .await?;
        let user = self.bearer_auth.auth(&req.params.headers).await?;
        let r = self.guest_requests_restore_token.handle(req, user).await?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }

    pub async fn guest_with_token_changes_password(
        &self,
        request: impl RequestTrait,
        raw_path_params: HashMap<&str, &str>,
    ) -> Result<HttpResponse, HttpError> {
        let raw_query_params = HashMap::from([]);
        let req: GuestWithTokenChangesPasswordRequest = self
            .request_transformer
            .transform(request, raw_path_params, raw_query_params)
            .await?;
        let user = self.bearer_auth.auth(&req.params.headers).await?;
        let r = self
            .guest_with_token_changes_password
            .handle(req, user)
            .await?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }

    pub async fn member_logins_with_password(
        &self,
        request: impl RequestTrait,
        raw_path_params: HashMap<&str, &str>,
    ) -> Result<HttpResponse, HttpError> {
        let raw_query_params = HashMap::from([]);
        let req: MemberLoginsWithPasswordRequest = self
            .request_transformer
            .transform(request, raw_path_params, raw_query_params)
            .await?;
        let user = self.bearer_auth.auth(&req.params.headers).await?;
        let r = self.member_logins_with_password.handle(req, user).await?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }

    pub async fn member_registers_with_password(
        &self,
        request: impl RequestTrait,
        raw_path_params: HashMap<&str, &str>,
    ) -> Result<HttpResponse, HttpError> {
        let raw_query_params = HashMap::from([]);
        let req: MemberRegistersWithPasswordRequest = self
            .request_transformer
            .transform(request, raw_path_params, raw_query_params)
            .await?;
        let user = self.bearer_auth.auth(&req.params.headers).await?;
        let r = self
            .member_registers_with_password
            .handle(req, user)
            .await?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }

    pub async fn member_verifies_email(
        &self,
        request: impl RequestTrait,
        raw_path_params: HashMap<&str, &str>,
    ) -> Result<HttpResponse, HttpError> {
        let raw_query_params = HashMap::from([]);
        let req: MemberVerifiesEmailRequest = self
            .request_transformer
            .transform(request, raw_path_params, raw_query_params)
            .await?;
        let user = self.bearer_auth.auth(&req.params.headers).await?;
        let r = self.member_verifies_email.handle(req, user).await?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }
}
