#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types
)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::task::{Context, Poll};
use swagger::{ApiError, ContextWrapper};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "/api/v1";
pub const API_VERSION: &str = "0.1.0-dev";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum AnonymousRegistersResponse {
    /// Created
    Created(models::AnonymousRegistrationResult),
    /// Bad request
    BadRequest(models::ProblemDetail),
    /// Unprocessable Entity
    UnprocessableEntity(models::ProblemDetail),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum AnonymousRequestsRestoreTokenResponse {
    /// OK
    OK(serde_json::Value),
    /// Bad request
    BadRequest(models::ProblemDetail),
    /// Unauthorized
    Unauthorized(models::ProblemDetail),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum AnonymousWithTokenChangesPasswordResponse {
    /// OK
    OK(serde_json::Value),
    /// Bad request
    BadRequest(models::ProblemDetail),
    /// Unauthorized
    Unauthorized(models::ProblemDetail),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UserLoginsWithPasswordResponse {
    /// OK
    OK(models::LoginResult),
    /// Bad request
    BadRequest(models::ProblemDetail),
    /// Unauthorized
    Unauthorized(models::ProblemDetail),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UserRegistersWithPasswordResponse {
    /// Created
    Created(models::RegistrationWithPasswordResult),
    /// Bad request
    BadRequest(models::ProblemDetail),
    /// Unprocessable Entity
    UnprocessableEntity(models::ProblemDetail),
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Anonymous registers
    async fn anonymous_registers(
        &self,
        context: &C,
    ) -> Result<AnonymousRegistersResponse, ApiError>;

    /// Anonymous requests restore password token
    async fn anonymous_requests_restore_token(
        &self,
        request_restore_token_schema: models::RequestRestoreTokenSchema,
        context: &C,
    ) -> Result<AnonymousRequestsRestoreTokenResponse, ApiError>;

    /// User with token changes his password
    async fn anonymous_with_token_changes_password(
        &self,
        change_password_schema: models::ChangePasswordSchema,
        context: &C,
    ) -> Result<AnonymousWithTokenChangesPasswordResponse, ApiError>;

    /// Login as an user
    async fn user_logins_with_password(
        &self,
        login_with_password_schema: models::LoginWithPasswordSchema,
        context: &C,
    ) -> Result<UserLoginsWithPasswordResponse, ApiError>;

    /// User registers with password
    async fn user_registers_with_password(
        &self,
        registration_with_password_schema: models::RegistrationWithPasswordSchema,
        context: &C,
    ) -> Result<UserRegistersWithPasswordResponse, ApiError>;
}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Anonymous registers
    async fn anonymous_registers(&self) -> Result<AnonymousRegistersResponse, ApiError>;

    /// Anonymous requests restore password token
    async fn anonymous_requests_restore_token(
        &self,
        request_restore_token_schema: models::RequestRestoreTokenSchema,
    ) -> Result<AnonymousRequestsRestoreTokenResponse, ApiError>;

    /// User with token changes his password
    async fn anonymous_with_token_changes_password(
        &self,
        change_password_schema: models::ChangePasswordSchema,
    ) -> Result<AnonymousWithTokenChangesPasswordResponse, ApiError>;

    /// Login as an user
    async fn user_logins_with_password(
        &self,
        login_with_password_schema: models::LoginWithPasswordSchema,
    ) -> Result<UserLoginsWithPasswordResponse, ApiError>;

    /// User registers with password
    async fn user_registers_with_password(
        &self,
        registration_with_password_schema: models::RegistrationWithPasswordSchema,
    ) -> Result<UserRegistersWithPasswordResponse, ApiError>;
}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync>
where
    Self: Sized,
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
        ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Anonymous registers
    async fn anonymous_registers(&self) -> Result<AnonymousRegistersResponse, ApiError> {
        let context = self.context().clone();
        self.api().anonymous_registers(&context).await
    }

    /// Anonymous requests restore password token
    async fn anonymous_requests_restore_token(
        &self,
        request_restore_token_schema: models::RequestRestoreTokenSchema,
    ) -> Result<AnonymousRequestsRestoreTokenResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .anonymous_requests_restore_token(request_restore_token_schema, &context)
            .await
    }

    /// User with token changes his password
    async fn anonymous_with_token_changes_password(
        &self,
        change_password_schema: models::ChangePasswordSchema,
    ) -> Result<AnonymousWithTokenChangesPasswordResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .anonymous_with_token_changes_password(change_password_schema, &context)
            .await
    }

    /// Login as an user
    async fn user_logins_with_password(
        &self,
        login_with_password_schema: models::LoginWithPasswordSchema,
    ) -> Result<UserLoginsWithPasswordResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .user_logins_with_password(login_with_password_schema, &context)
            .await
    }

    /// User registers with password
    async fn user_registers_with_password(
        &self,
        registration_with_password_schema: models::RegistrationWithPasswordSchema,
    ) -> Result<UserRegistersWithPasswordResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .user_registers_with_password(registration_with_password_schema, &context)
            .await
    }
}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
