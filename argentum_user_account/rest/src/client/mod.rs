use crate::dto::response::AnonymousRegisteredSuccessfullyResponse;
use crate::dto::response::EmptyOkResponse;
use crate::dto::response::Status400Response;
use crate::dto::response::Status401Response;
use crate::dto::response::Status409Response;
use crate::dto::response::UserLoggedInSuccessfullyResponse;
use crate::dto::response::UserRegisteredSuccessfullyResponse;
use crate::dto::schema::AnonymousRegistrationResult as AnonymousRegistrationResultSchema;
use crate::dto::schema::EmptyResponse as EmptyResponseSchema;
use crate::dto::schema::LoginResult as LoginResultSchema;
use crate::dto::schema::ProblemDetail as ProblemDetailSchema;
use crate::dto::schema::RegistrationWithPasswordResult as RegistrationWithPasswordResultSchema;

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
    ) -> Result<AnonymousRegistersOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/user-account/anonymous-register".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body).unwrap();
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform AnonymousRegistersRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::CREATED => {
                    match response.json::<AnonymousRegistrationResultSchema>().await {
                        Ok(data) => Ok(AnonymousRegistersOperationResponseEnum::Status201(
                            AnonymousRegisteredSuccessfullyResponse::new_application_json(data),
                        )),
                        Err(e) => Err(e.to_string()),
                    }
                }

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn user_logins_with_password(
        &self,
        req: UserLoginsWithPasswordRequest,
    ) -> Result<UserLoginsWithPasswordOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/user-account/password-login".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body).unwrap();
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform UserLoginsWithPasswordRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            //TODO: add support of another authorization schemas
            .header(
                "authorization",
                format!("Bearer {}", req.params.headers.authorization),
            )
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<LoginResultSchema>().await {
                    Ok(data) => Ok(UserLoginsWithPasswordOperationResponseEnum::Status200(
                        UserLoggedInSuccessfullyResponse::new_application_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(UserLoginsWithPasswordOperationResponseEnum::Status400(
                        Status400Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ProblemDetailSchema>().await {
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
    ) -> Result<UserRegistersWithPasswordOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/user-account/register".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body).unwrap();
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform UserRegistersWithPasswordRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            //TODO: add support of another authorization schemas
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
                    match response
                        .json::<RegistrationWithPasswordResultSchema>()
                        .await
                    {
                        Ok(data) => Ok(UserRegistersWithPasswordOperationResponseEnum::Status201(
                            UserRegisteredSuccessfullyResponse::new_application_json(data),
                        )),
                        Err(e) => Err(e.to_string()),
                    }
                }
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(UserRegistersWithPasswordOperationResponseEnum::Status400(
                        Status400Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::CONFLICT => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(UserRegistersWithPasswordOperationResponseEnum::Status409(
                        Status409Response::new_application_problem_json(data),
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
    ) -> Result<AnonymousRequestsRestoreTokenOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/user-account/restore-password/token-request".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body).unwrap();
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform AnonymousRequestsRestoreTokenRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            //TODO: add support of another authorization schemas
            .header(
                "authorization",
                format!("Bearer {}", req.params.headers.authorization),
            )
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<EmptyResponseSchema>().await {
                    Ok(data) => Ok(
                        AnonymousRequestsRestoreTokenOperationResponseEnum::Status200(
                            EmptyOkResponse::new_application_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(
                        AnonymousRequestsRestoreTokenOperationResponseEnum::Status400(
                            Status400Response::new_application_problem_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ProblemDetailSchema>().await {
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
    ) -> Result<AnonymousWithTokenChangesPasswordOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/user/restore-password/change-password".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body).unwrap();
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform AnonymousWithTokenChangesPasswordRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            //TODO: add support of another authorization schemas
            .header(
                "authorization",
                format!("Bearer {}", req.params.headers.authorization),
            )
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<EmptyResponseSchema>().await {
                    Ok(data) => Ok(
                        AnonymousWithTokenChangesPasswordOperationResponseEnum::Status200(
                            EmptyOkResponse::new_application_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(
                        AnonymousWithTokenChangesPasswordOperationResponseEnum::Status400(
                            Status400Response::new_application_problem_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ProblemDetailSchema>().await {
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
