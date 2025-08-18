use crate::dto::response::GetUserOkResponse;
use crate::dto::response::Status401Response;
use crate::dto::response::Status403Response;
use crate::dto::response::Status404Response;

use crate::dto::schema::ProblemDetail as ProblemDetailSchema;
use crate::dto::schema::User as UserSchema;

use crate::dto::operation_response_enum::GetUserOperationResponseEnum;
use crate::dto::request::GetUserRequest;
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

    pub async fn get_user(
        &self,
        req: GetUserRequest,
    ) -> Result<GetUserOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let mut url_tpl = "/user/{userId}".to_string();

        url_tpl = url_tpl.replace("{userId}", req.params.path.user_id.to_string().as_str());

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform GetUserRequest into reqwest object
        let res = client
            .get(url)
            .header("Accept", "application/json")
            //TODO: add support of another authorization schemas
            .header(
                "authorization",
                format!("Bearer {}", req.params.headers.authorization),
            )
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<UserSchema>().await {
                    Ok(data) => Ok(GetUserOperationResponseEnum::Status200(
                        GetUserOkResponse::new_application_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::UNAUTHORIZED => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(GetUserOperationResponseEnum::Status401(
                        Status401Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::FORBIDDEN => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(GetUserOperationResponseEnum::Status403(
                        Status403Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::NOT_FOUND => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(GetUserOperationResponseEnum::Status404(
                        Status404Response::new_application_problem_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },

                _ => Err("Wrong status code".to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
