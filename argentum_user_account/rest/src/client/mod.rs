use crate::dto::response::AnonymousRegisteredSuccessfullyResponse;
use crate::dto::response::EmptyOkResponse;
use crate::dto::response::Status400Response;
use crate::dto::response::Status401Response;
use crate::dto::response::Status422Response;
use crate::dto::response::UserLoggedInSuccessfullyResponse;
use crate::dto::response::UserRegisteredSuccessfullyResponse;
use crate::dto::schema::AnonymousRegistrationResult;
use crate::dto::schema::EmptyResponse;
use crate::dto::schema::LoginResult;
use crate::dto::schema::ProblemDetail;
use crate::dto::schema::RegistrationWithPasswordResult;

use crate::dto::operation_response_enum::AnonymousRegistersOperationResponseEnum;
use crate::dto::operation_response_enum::AnonymousRequestsRestoreTokenOperationResponseEnum;
use crate::dto::operation_response_enum::AnonymousWithTokenChangesPasswordOperationResponseEnum;
use crate::dto::operation_response_enum::UserLoginsWithPasswordOperationResponseEnum;
use crate::dto::operation_response_enum::UserRegistersWithPasswordOperationResponseEnum;
use crate::dto::request::AnonymousRegistersRequest;
use crate::dto::request::AnonymousRequestsRestoreTokenRequest;
use crate::dto::request::AnonymousWithTokenChangesPasswordRequest;
use crate::dto::request::UserLoginsWithPasswordRequest;
use crate::dto::request::UserRegistersWithPasswordRequest;

use reqwest::StatusCode;

pub struct Client {
    server_url: String,
    base_path: String,
}

impl Client {
    pub fn new(server_url: String, base_path: String) -> Self {
        Self {
            server_url,
            base_path,
        }
    }

    pub async fn anonymous_registers(
        &self,
        req: AnonymousRegistersRequest,
        //TODO: need some way to deal with anonymous/authorized users
    ) -> Result<AnonymousRegistersOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        //let raw_query_params = HashMap::from([]);
        //let req: AnonymousRegistersRequest = self
        //    .request_transformer
        //    .transform(request, raw_path_params, raw_query_params)
        //    .await?;
        //    let r = self.anonymous_registers.handle(req)?;

        let client = reqwest::Client::new();

        let url = format!(
            "{}{}/user-account/anonymous-register",
            self.server_url, self.base_path,
        );

        let body = serde_json::to_vec_pretty(&req.body).unwrap();
        //TODO: params: header, query, path, auth

        //TODO: transform AnonymousRegistersRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::CREATED => match response.json::<AnonymousRegistrationResult>().await {
                    Ok(data) => Ok(AnonymousRegistersOperationResponseEnum::Status201(
                        AnonymousRegisteredSuccessfullyResponse::new_application_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn user_logins_with_password(
        &self,
        req: UserLoginsWithPasswordRequest,
        //TODO: need some way to deal with anonymous/authorized users
    ) -> Result<UserLoginsWithPasswordOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        //let raw_query_params = HashMap::from([]);
        //let req: UserLoginsWithPasswordRequest = self
        //    .request_transformer
        //    .transform(request, raw_path_params, raw_query_params)
        //    .await?;
        //TODO: deal with security

        let client = reqwest::Client::new();

        let url = format!(
            "{}{}/user-account/password-login",
            self.server_url, self.base_path,
        );

        let body = serde_json::to_vec_pretty(&req.body).unwrap();
        //TODO: params: header, query, path, auth

        //TODO: transform UserLoginsWithPasswordRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            .header(
                "authorization",
                format!("Bearer {}", req.params.headers.authorization),
            )
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<LoginResult>().await {
                    Ok(data) => Ok(UserLoginsWithPasswordOperationResponseEnum::Status200(
                        UserLoggedInSuccessfullyResponse::new_application_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetail>().await {
                    Ok(data) => Ok(UserLoginsWithPasswordOperationResponseEnum::Status400(
                        Status400Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ProblemDetail>().await {
                    Ok(data) => Ok(UserLoginsWithPasswordOperationResponseEnum::Status401(
                        Status401Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn user_registers_with_password(
        &self,
        req: UserRegistersWithPasswordRequest,
        //TODO: need some way to deal with anonymous/authorized users
    ) -> Result<UserRegistersWithPasswordOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        //let raw_query_params = HashMap::from([]);
        //let req: UserRegistersWithPasswordRequest = self
        //    .request_transformer
        //    .transform(request, raw_path_params, raw_query_params)
        //    .await?;
        //TODO: deal with security

        let client = reqwest::Client::new();

        let url = format!(
            "{}{}/user-account/register",
            self.server_url, self.base_path,
        );

        let body = serde_json::to_vec_pretty(&req.body).unwrap();
        //TODO: params: header, query, path, auth

        //TODO: transform UserRegistersWithPasswordRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            .header(
                "authorization",
                format!("Bearer {}", req.params.headers.authorization),
            )
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::CREATED => {
                    match response.json::<RegistrationWithPasswordResult>().await {
                        Ok(data) => Ok(UserRegistersWithPasswordOperationResponseEnum::Status201(
                            UserRegisteredSuccessfullyResponse::new_application_json(data),
                        )),
                        Err(e) => Err(e.to_string()),
                    }
                }
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetail>().await {
                    Ok(data) => Ok(UserRegistersWithPasswordOperationResponseEnum::Status400(
                        Status400Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNPROCESSABLE_ENTITY => match response.json::<ProblemDetail>().await {
                    Ok(data) => Ok(UserRegistersWithPasswordOperationResponseEnum::Status422(
                        Status422Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn anonymous_requests_restore_token(
        &self,
        req: AnonymousRequestsRestoreTokenRequest,
        //TODO: need some way to deal with anonymous/authorized users
    ) -> Result<AnonymousRequestsRestoreTokenOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        //let raw_query_params = HashMap::from([]);
        //let req: AnonymousRequestsRestoreTokenRequest = self
        //    .request_transformer
        //    .transform(request, raw_path_params, raw_query_params)
        //    .await?;
        //TODO: deal with security

        let client = reqwest::Client::new();

        let url = format!(
            "{}{}/user-account/restore-password/token-request",
            self.server_url, self.base_path,
        );

        let body = serde_json::to_vec_pretty(&req.body).unwrap();
        //TODO: params: header, query, path, auth

        //TODO: transform AnonymousRequestsRestoreTokenRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            .header(
                "authorization",
                format!("Bearer {}", req.params.headers.authorization),
            )
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<EmptyResponse>().await {
                    Ok(data) => Ok(
                        AnonymousRequestsRestoreTokenOperationResponseEnum::Status200(
                            EmptyOkResponse::new_application_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetail>().await {
                    Ok(data) => Ok(
                        AnonymousRequestsRestoreTokenOperationResponseEnum::Status400(
                            Status400Response::new_application_problem_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ProblemDetail>().await {
                    Ok(data) => Ok(
                        AnonymousRequestsRestoreTokenOperationResponseEnum::Status401(
                            Status401Response::new_application_problem_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn anonymous_with_token_changes_password(
        &self,
        req: AnonymousWithTokenChangesPasswordRequest,
        //TODO: need some way to deal with anonymous/authorized users
    ) -> Result<AnonymousWithTokenChangesPasswordOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        //let raw_query_params = HashMap::from([]);
        //let req: AnonymousWithTokenChangesPasswordRequest = self
        //    .request_transformer
        //    .transform(request, raw_path_params, raw_query_params)
        //    .await?;
        //TODO: deal with security

        let client = reqwest::Client::new();

        let url = format!(
            "{}{}/user/restore-password/change-password",
            self.server_url, self.base_path,
        );

        let body = serde_json::to_vec_pretty(&req.body).unwrap();
        //TODO: params: header, query, path, auth

        //TODO: transform AnonymousWithTokenChangesPasswordRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            .header(
                "authorization",
                format!("Bearer {}", req.params.headers.authorization),
            )
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<EmptyResponse>().await {
                    Ok(data) => Ok(
                        AnonymousWithTokenChangesPasswordOperationResponseEnum::Status200(
                            EmptyOkResponse::new_application_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetail>().await {
                    Ok(data) => Ok(
                        AnonymousWithTokenChangesPasswordOperationResponseEnum::Status400(
                            Status400Response::new_application_problem_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ProblemDetail>().await {
                    Ok(data) => Ok(
                        AnonymousWithTokenChangesPasswordOperationResponseEnum::Status401(
                            Status401Response::new_application_problem_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
