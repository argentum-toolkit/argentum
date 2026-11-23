use crate::dto::response::MessageCreatedSuccessfullyResponse;
use crate::dto::response::Status400Response;

use crate::dto::schema::MessageCreated as MessageCreatedSchema;
use crate::dto::schema::ProblemDetail as ProblemDetailSchema;

use crate::dto::operation_response_enum::GuestSendsMessageOperationResponseEnum;
use crate::dto::operation_response_enum::MemberSendsMessageOperationResponseEnum;
use crate::dto::request::GuestSendsMessageRequest;
use crate::dto::request::MemberSendsMessageRequest;
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

    pub async fn guest_sends_message(
        &self,
        req: GuestSendsMessageRequest,
    ) -> Result<GuestSendsMessageOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/contact-us/guest-message".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body)
            .map_err(|e| format!("Can't parse request body. Error: {e}"))?;
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform GuestSendsMessageRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::CREATED => match response.json::<MessageCreatedSchema>().await {
                    Ok(data) => Ok(GuestSendsMessageOperationResponseEnum::Status201(
                        MessageCreatedSuccessfullyResponse::new_application_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(GuestSendsMessageOperationResponseEnum::Status400(
                        Status400Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn member_sends_message(
        &self,
        req: MemberSendsMessageRequest,
    ) -> Result<MemberSendsMessageOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/contact-us/member-message".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body)
            .map_err(|e| format!("Can't parse request body. Error: {e}"))?;
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform MemberSendsMessageRequest into reqwest object
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
                StatusCode::CREATED => match response.json::<MessageCreatedSchema>().await {
                    Ok(data) => Ok(MemberSendsMessageOperationResponseEnum::Status201(
                        MessageCreatedSuccessfullyResponse::new_application_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(MemberSendsMessageOperationResponseEnum::Status400(
                        Status400Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
