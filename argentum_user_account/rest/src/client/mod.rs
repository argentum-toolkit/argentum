use crate::dto::response::EmptyOkResponse;
use crate::dto::response::GuestRegisteredSuccessfullyResponse;
use crate::dto::response::MemberLoggedInSuccessfullyResponse;
use crate::dto::response::MemberRegisteredSuccessfullyResponse;
use crate::dto::response::Status400Response;
use crate::dto::response::Status401Response;
use crate::dto::response::Status409Response;

use crate::dto::schema::EmptyResponse as EmptyResponseSchema;
use crate::dto::schema::GuestRegistrationResult as GuestRegistrationResultSchema;
use crate::dto::schema::LoginResult as LoginResultSchema;
use crate::dto::schema::ProblemDetail as ProblemDetailSchema;
use crate::dto::schema::RegistrationWithPasswordResult as RegistrationWithPasswordResultSchema;

use crate::dto::operation_response_enum::GuestRegistersOperationResponseEnum;
use crate::dto::operation_response_enum::GuestRequestsRestoreTokenOperationResponseEnum;
use crate::dto::operation_response_enum::GuestWithTokenChangesPasswordOperationResponseEnum;
use crate::dto::operation_response_enum::MemberLoginsWithPasswordOperationResponseEnum;
use crate::dto::operation_response_enum::MemberRegistersWithPasswordOperationResponseEnum;
use crate::dto::operation_response_enum::MemberVerifiesEmailOperationResponseEnum;
use crate::dto::request::GuestRegistersRequest;
use crate::dto::request::GuestRequestsRestoreTokenRequest;
use crate::dto::request::GuestWithTokenChangesPasswordRequest;
use crate::dto::request::MemberLoginsWithPasswordRequest;
use crate::dto::request::MemberRegistersWithPasswordRequest;
use crate::dto::request::MemberVerifiesEmailRequest;
use reqwest::StatusCode;

pub struct Client {
    server_url: String,
    base_path: String,
}

impl Client {
    pub fn new(server_url: &str, base_path: &str) -> Self {
        Self {
            server_url: server_url.into(),
            base_path: base_path.into(),
        }
    }

    pub async fn member_verifies_email(
        &self,
        req: MemberVerifiesEmailRequest,
    ) -> Result<MemberVerifiesEmailOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/user-account/email-verification".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body)
            .map_err(|e| format!("Can't parse request body. Error: {e}"))?;
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform MemberVerifiesEmailRequest into reqwest object
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
                    Ok(data) => Ok(MemberVerifiesEmailOperationResponseEnum::Status200(
                        EmptyOkResponse::new_application_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(MemberVerifiesEmailOperationResponseEnum::Status400(
                        Status400Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(MemberVerifiesEmailOperationResponseEnum::Status401(
                        Status401Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn guest_registers(
        &self,
        req: GuestRegistersRequest,
    ) -> Result<GuestRegistersOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/user-account/guest-registration".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body)
            .map_err(|e| format!("Can't parse request body. Error: {e}"))?;
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform GuestRegistersRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::CREATED => {
                    match response.json::<GuestRegistrationResultSchema>().await {
                        Ok(data) => Ok(GuestRegistersOperationResponseEnum::Status201(
                            GuestRegisteredSuccessfullyResponse::new_application_json(data),
                        )),
                        Err(e) => Err(e.to_string()),
                    }
                }

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn member_logins_with_password(
        &self,
        req: MemberLoginsWithPasswordRequest,
    ) -> Result<MemberLoginsWithPasswordOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/user-account/password-login".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body)
            .map_err(|e| format!("Can't parse request body. Error: {e}"))?;
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform MemberLoginsWithPasswordRequest into reqwest object
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
                    Ok(data) => Ok(MemberLoginsWithPasswordOperationResponseEnum::Status200(
                        MemberLoggedInSuccessfullyResponse::new_application_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(MemberLoginsWithPasswordOperationResponseEnum::Status400(
                        Status400Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(MemberLoginsWithPasswordOperationResponseEnum::Status401(
                        Status401Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn member_registers_with_password(
        &self,
        req: MemberRegistersWithPasswordRequest,
    ) -> Result<MemberRegistersWithPasswordOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/user-account/register".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body)
            .map_err(|e| format!("Can't parse request body. Error: {e}"))?;
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform MemberRegistersWithPasswordRequest into reqwest object
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
                        Ok(data) => {
                            Ok(MemberRegistersWithPasswordOperationResponseEnum::Status201(
                                MemberRegisteredSuccessfullyResponse::new_application_json(data),
                            ))
                        }
                        Err(e) => Err(e.to_string()),
                    }
                }
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(MemberRegistersWithPasswordOperationResponseEnum::Status400(
                        Status400Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::CONFLICT => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(MemberRegistersWithPasswordOperationResponseEnum::Status409(
                        Status409Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn guest_with_token_changes_password(
        &self,
        req: GuestWithTokenChangesPasswordRequest,
    ) -> Result<GuestWithTokenChangesPasswordOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/user-account/restore-password/change-password".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body)
            .map_err(|e| format!("Can't parse request body. Error: {e}"))?;
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform GuestWithTokenChangesPasswordRequest into reqwest object
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
                        GuestWithTokenChangesPasswordOperationResponseEnum::Status200(
                            EmptyOkResponse::new_application_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(
                        GuestWithTokenChangesPasswordOperationResponseEnum::Status400(
                            Status400Response::new_application_problem_json(data),
                        ),
                    ),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(
                        GuestWithTokenChangesPasswordOperationResponseEnum::Status401(
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

    pub async fn guest_requests_restore_token(
        &self,
        req: GuestRequestsRestoreTokenRequest,
    ) -> Result<GuestRequestsRestoreTokenOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/user-account/restore-password/token-request".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body)
            .map_err(|e| format!("Can't parse request body. Error: {e}"))?;
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform GuestRequestsRestoreTokenRequest into reqwest object
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
                    Ok(data) => Ok(GuestRequestsRestoreTokenOperationResponseEnum::Status200(
                        EmptyOkResponse::new_application_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(GuestRequestsRestoreTokenOperationResponseEnum::Status400(
                        Status400Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(GuestRequestsRestoreTokenOperationResponseEnum::Status401(
                        Status401Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
