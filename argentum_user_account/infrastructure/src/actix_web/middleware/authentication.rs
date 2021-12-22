use actix_http::header;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpMessage, HttpResponse};
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::actix_web::http_problem::{
    build_internal_server_error_response, HttpProblemError,
};
use argentum_standard_infrastructure::error::InternalError;
use argentum_user_account_business::use_case::user_authenticates_with_token::AuthenticationError as BusinessAuthenticationError;
use argentum_user_account_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use derive_more::{Display, Error};
use futures_util::future::{err, ok, Ready};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct AuthenticationMiddlewareFactory;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for AuthenticationMiddlewareFactory
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if "/api/v1/user/anonymous-register" != req.path() {
            let header: String = match req.headers().get(header::AUTHORIZATION) {
                Some(header) => match header.to_str() {
                    Ok(header) => header.to_string(),
                    Err(_) => {
                        return Box::pin(async move {
                            err(AuthenticationError {
                                details: "can't read `Authorization` header",
                            })
                            .await?
                        });
                    }
                },
                None => {
                    return Box::pin(async move {
                        err(AuthenticationError {
                            details: "`Authorisation` header is not found",
                        })
                        .await?
                    });
                }
            };

            // header.contains("Bearer ").;
            match header.find("Bearer ") {
                Some(index) => match index {
                    0 => (),
                    _ => {
                        return Box::pin(async move {
                            err(AuthenticationError { details: "The expected format of `Authorization` header is `Bearer {token}`" }).await?
                        })
                    }
                },
                _ => {
                    return Box::pin(async move {
                        err(AuthenticationError {
                            details:
                                "The expected format of `Authorization` header is `Bearer {token}`",
                        })
                        .await?
                    });
                }
            }

            let token: String = header.chars().skip(7).collect();

            let uc = req
                .app_data::<web::Data<Arc<UserAuthenticatesWithTokenUc>>>()
                .unwrap();

            let logger = req.app_data::<web::Data<Arc<dyn LoggerTrait>>>().unwrap();

            let user = match uc.execute(token) {
                Ok(user) => user,
                Err(BusinessAuthenticationError::UserNotFound)
                | Err(BusinessAuthenticationError::WrongToken) => {
                    return Box::pin(async move {
                        err(AuthenticationError {
                            details: "can't authenticate",
                        })
                        .await?
                    });
                }
                Err(e) => {
                    logger.error(format!("{:?}", InternalError::Server(Box::new(e))));

                    return Box::pin(
                        async move { err(build_internal_server_error_response()).await? },
                    );
                }
            };

            req.extensions_mut().insert(user);
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            Ok(res)
        })
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Unauthorized: {}", details)]
struct AuthenticationError {
    details: &'static str,
}

impl ResponseError for AuthenticationError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized()
            .content_type("application/problem+json")
            .json(HttpProblemError::new(
                self.to_string(),
                StatusCode::UNAUTHORIZED.as_u16(),
            ))
    }
}
