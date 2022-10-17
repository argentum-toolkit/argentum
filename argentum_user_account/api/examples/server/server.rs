//! Main library entry point for argentum_user_account_api implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use swagger::{Has, XSpanIdString};
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use argentum_user_account_api::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        argentum_user_account_api::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls())
                .expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM)
                .expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem")
                .expect("Failed to set certificate chain");
            ssl.check_private_key()
                .expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr)
            .serve(service)
            .await
            .unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server {
            marker: PhantomData,
        }
    }
}

use argentum_user_account_api::server::MakeService;
use argentum_user_account_api::{
    AnonymousRegistersResponse, Api, ChangePasswordWithTokenResponse, LoginWithPasswordResponse,
    RegisterWithPasswordResponse, RequestRestoreTokenResponse,
};
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    /// Anonymous registers
    async fn anonymous_registers(
        &self,
        body: Option<serde_json::Value>,
        context: &C,
    ) -> Result<AnonymousRegistersResponse, ApiError> {
        let context = context.clone();
        info!(
            "anonymous_registers({:?}) - X-Span-ID: {:?}",
            body,
            context.get().0.clone()
        );
        Err(ApiError("Generic failure".into()))
    }

    /// User with token changes his password
    async fn change_password_with_token(
        &self,
        change_password_schema: models::ChangePasswordSchema,
        context: &C,
    ) -> Result<ChangePasswordWithTokenResponse, ApiError> {
        let context = context.clone();
        info!(
            "change_password_with_token({:?}) - X-Span-ID: {:?}",
            change_password_schema,
            context.get().0.clone()
        );
        Err(ApiError("Generic failure".into()))
    }

    /// Login as an user
    async fn login_with_password(
        &self,
        login_with_password_schema: models::LoginWithPasswordSchema,
        context: &C,
    ) -> Result<LoginWithPasswordResponse, ApiError> {
        let context = context.clone();
        info!(
            "login_with_password({:?}) - X-Span-ID: {:?}",
            login_with_password_schema,
            context.get().0.clone()
        );
        Err(ApiError("Generic failure".into()))
    }

    /// User registers with password
    async fn register_with_password(
        &self,
        registration_with_password_schema: models::RegistrationWithPasswordSchema,
        context: &C,
    ) -> Result<RegisterWithPasswordResponse, ApiError> {
        let context = context.clone();
        info!(
            "register_with_password({:?}) - X-Span-ID: {:?}",
            registration_with_password_schema,
            context.get().0.clone()
        );
        Err(ApiError("Generic failure".into()))
    }

    /// Anonymous requests restore password token
    async fn request_restore_token(
        &self,
        request_restore_token_schema: models::RequestRestoreTokenSchema,
        context: &C,
    ) -> Result<RequestRestoreTokenResponse, ApiError> {
        let context = context.clone();
        info!(
            "request_restore_token({:?}) - X-Span-ID: {:?}",
            request_restore_token_schema,
            context.get().0.clone()
        );
        Err(ApiError("Generic failure".into()))
    }
}
