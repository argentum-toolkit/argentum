use futures::{future, future::BoxFuture, future::FutureExt, stream, stream::TryStreamExt, Stream};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use hyper::{Body, HeaderMap, Request, Response, StatusCode};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
use url::form_urlencoded;

use crate::header;
#[allow(unused_imports)]
use crate::models;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{
    AnonymousRegistersResponse, AnonymousRequestsRestoreTokenResponse,
    AnonymousWithTokenChangesPasswordResponse, Api, UserLoginsWithPasswordResponse,
    UserRegistersWithPasswordResponse,
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/api/v1/user/anonymous-register$",
            r"^/api/v1/user/password-login$",
            r"^/api/v1/user/register$",
            r"^/api/v1/user/restore-password/change-password$",
            r"^/api/v1/user/restore-password/token-request$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_USER_ANONYMOUS_REGISTER: usize = 0;
    pub(crate) static ID_USER_PASSWORD_LOGIN: usize = 1;
    pub(crate) static ID_USER_REGISTER: usize = 2;
    pub(crate) static ID_USER_RESTORE_PASSWORD_CHANGE_PASSWORD: usize = 3;
    pub(crate) static ID_USER_RESTORE_PASSWORD_TOKEN_REQUEST: usize = 4;
}

pub struct MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData,
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(self.api_impl.clone()))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::empty())
        .expect("Unable to create Method Not Allowed response"))
}

pub struct Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData,
        }
    }
}

impl<T, C> Clone for Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C>
where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future {
        async fn run<T, C>(
            mut api_impl: T,
            req: (Request<Body>, C),
        ) -> Result<Response<Body>, crate::ServiceError>
        where
            T: Api<C> + Clone + Send + 'static,
            C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
        {
            let (request, context) = req;
            let (parts, body) = request.into_parts();
            let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
            let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

            match method {
                // AnonymousRegisters - POST /user/anonymous-register
                hyper::Method::POST if path.matched(paths::ID_USER_ANONYMOUS_REGISTER) => {
                    let result = api_impl.anonymous_registers(&context).await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            AnonymousRegistersResponse::Created(body) => {
                                *response.status_mut() = StatusCode::from_u16(201)
                                    .expect("Unable to turn 201 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ANONYMOUS_REGISTERS_CREATED"));
                                let body = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body);
                            }
                            AnonymousRegistersResponse::BadRequest(body) => {
                                *response.status_mut() = StatusCode::from_u16(400)
                                    .expect("Unable to turn 400 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+json")
                                                            .expect("Unable to create Content-Type header for ANONYMOUS_REGISTERS_BAD_REQUEST"));
                                let body = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body);
                            }
                            AnonymousRegistersResponse::UnprocessableEntity(body) => {
                                *response.status_mut() = StatusCode::from_u16(422)
                                    .expect("Unable to turn 422 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+json")
                                                            .expect("Unable to create Content-Type header for ANONYMOUS_REGISTERS_UNPROCESSABLE_ENTITY"));
                                let body = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body);
                            }
                        },
                        Err(_) => {
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // AnonymousRequestsRestoreToken - POST /user/restore-password/token-request
                hyper::Method::POST
                    if path.matched(paths::ID_USER_RESTORE_PASSWORD_TOKEN_REQUEST) =>
                {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_request_restore_token_schema: Option<models::RequestRestoreTokenSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_request_restore_token_schema) => param_request_restore_token_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter RequestRestoreTokenSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter RequestRestoreTokenSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_request_restore_token_schema = match param_request_restore_token_schema {
                                    Some(param_request_restore_token_schema) => param_request_restore_token_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter RequestRestoreTokenSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter RequestRestoreTokenSchema")),
                                };

                                let result = api_impl.anonymous_requests_restore_token(
                                            param_request_restore_token_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AnonymousRequestsRestoreTokenResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ANONYMOUS_REQUESTS_RESTORE_TOKEN_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                AnonymousRequestsRestoreTokenResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+json")
                                                            .expect("Unable to create Content-Type header for ANONYMOUS_REQUESTS_RESTORE_TOKEN_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                AnonymousRequestsRestoreTokenResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+json")
                                                            .expect("Unable to create Content-Type header for ANONYMOUS_REQUESTS_RESTORE_TOKEN_UNAUTHORIZED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter RequestRestoreTokenSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter RequestRestoreTokenSchema")),
                        }
                }

                // AnonymousWithTokenChangesPassword - POST /user/restore-password/change-password
                hyper::Method::POST
                    if path.matched(paths::ID_USER_RESTORE_PASSWORD_CHANGE_PASSWORD) =>
                {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_change_password_schema: Option<models::ChangePasswordSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_change_password_schema) => param_change_password_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter ChangePasswordSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter ChangePasswordSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_change_password_schema = match param_change_password_schema {
                                    Some(param_change_password_schema) => param_change_password_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter ChangePasswordSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter ChangePasswordSchema")),
                                };

                                let result = api_impl.anonymous_with_token_changes_password(
                                            param_change_password_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AnonymousWithTokenChangesPasswordResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ANONYMOUS_WITH_TOKEN_CHANGES_PASSWORD_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                AnonymousWithTokenChangesPasswordResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+json")
                                                            .expect("Unable to create Content-Type header for ANONYMOUS_WITH_TOKEN_CHANGES_PASSWORD_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                AnonymousWithTokenChangesPasswordResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+json")
                                                            .expect("Unable to create Content-Type header for ANONYMOUS_WITH_TOKEN_CHANGES_PASSWORD_UNAUTHORIZED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter ChangePasswordSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter ChangePasswordSchema")),
                        }
                }

                // UserLoginsWithPassword - POST /user/password-login
                hyper::Method::POST if path.matched(paths::ID_USER_PASSWORD_LOGIN) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_login_with_password_schema: Option<models::LoginWithPasswordSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_login_with_password_schema) => param_login_with_password_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter LoginWithPasswordSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter LoginWithPasswordSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_login_with_password_schema = match param_login_with_password_schema {
                                    Some(param_login_with_password_schema) => param_login_with_password_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter LoginWithPasswordSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter LoginWithPasswordSchema")),
                                };

                                let result = api_impl.user_logins_with_password(
                                            param_login_with_password_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UserLoginsWithPasswordResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for USER_LOGINS_WITH_PASSWORD_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                UserLoginsWithPasswordResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+json")
                                                            .expect("Unable to create Content-Type header for USER_LOGINS_WITH_PASSWORD_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                UserLoginsWithPasswordResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+json")
                                                            .expect("Unable to create Content-Type header for USER_LOGINS_WITH_PASSWORD_UNAUTHORIZED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter LoginWithPasswordSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter LoginWithPasswordSchema")),
                        }
                }

                // UserRegistersWithPassword - POST /user/register
                hyper::Method::POST if path.matched(paths::ID_USER_REGISTER) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_registration_with_password_schema: Option<models::RegistrationWithPasswordSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_registration_with_password_schema) => param_registration_with_password_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter RegistrationWithPasswordSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter RegistrationWithPasswordSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_registration_with_password_schema = match param_registration_with_password_schema {
                                    Some(param_registration_with_password_schema) => param_registration_with_password_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter RegistrationWithPasswordSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter RegistrationWithPasswordSchema")),
                                };

                                let result = api_impl.user_registers_with_password(
                                            param_registration_with_password_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UserRegistersWithPasswordResponse::Created
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for USER_REGISTERS_WITH_PASSWORD_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                UserRegistersWithPasswordResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+json")
                                                            .expect("Unable to create Content-Type header for USER_REGISTERS_WITH_PASSWORD_BAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                UserRegistersWithPasswordResponse::UnprocessableEntity
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/problem+json")
                                                            .expect("Unable to create Content-Type header for USER_REGISTERS_WITH_PASSWORD_UNPROCESSABLE_ENTITY"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter RegistrationWithPasswordSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter RegistrationWithPasswordSchema")),
                        }
                }

                _ if path.matched(paths::ID_USER_ANONYMOUS_REGISTER) => method_not_allowed(),
                _ if path.matched(paths::ID_USER_PASSWORD_LOGIN) => method_not_allowed(),
                _ if path.matched(paths::ID_USER_REGISTER) => method_not_allowed(),
                _ if path.matched(paths::ID_USER_RESTORE_PASSWORD_CHANGE_PASSWORD) => {
                    method_not_allowed()
                }
                _ if path.matched(paths::ID_USER_RESTORE_PASSWORD_TOKEN_REQUEST) => {
                    method_not_allowed()
                }
                _ => Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response")),
            }
        }
        Box::pin(run(self.api_impl.clone(), req))
    }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // AnonymousRegisters - POST /user/anonymous-register
            hyper::Method::POST if path.matched(paths::ID_USER_ANONYMOUS_REGISTER) => {
                Some("AnonymousRegisters")
            }
            // AnonymousRequestsRestoreToken - POST /user/restore-password/token-request
            hyper::Method::POST if path.matched(paths::ID_USER_RESTORE_PASSWORD_TOKEN_REQUEST) => {
                Some("AnonymousRequestsRestoreToken")
            }
            // AnonymousWithTokenChangesPassword - POST /user/restore-password/change-password
            hyper::Method::POST
                if path.matched(paths::ID_USER_RESTORE_PASSWORD_CHANGE_PASSWORD) =>
            {
                Some("AnonymousWithTokenChangesPassword")
            }
            // UserLoginsWithPassword - POST /user/password-login
            hyper::Method::POST if path.matched(paths::ID_USER_PASSWORD_LOGIN) => {
                Some("UserLoginsWithPassword")
            }
            // UserRegistersWithPassword - POST /user/register
            hyper::Method::POST if path.matched(paths::ID_USER_REGISTER) => {
                Some("UserRegistersWithPassword")
            }
            _ => None,
        }
    }
}
