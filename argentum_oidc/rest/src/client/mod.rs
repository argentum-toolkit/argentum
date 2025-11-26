use crate::dto::response::CodeExchangedSuccessfullyResponse;
use crate::dto::response::Status400Response;

use crate::dto::schema::ExchangeCodeResult as ExchangeCodeResultSchema;
use crate::dto::schema::ProblemDetail as ProblemDetailSchema;

use crate::dto::operation_response_enum::ExchangeCodeOperationResponseEnum;
use crate::dto::request::ExchangeCodeRequest;
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

    pub async fn exchange_code(
        &self,
        req: ExchangeCodeRequest,
    ) -> Result<ExchangeCodeOperationResponseEnum, String> {
        //TODO: use better type instead of Err(String)

        let url_tpl = "/exchange-code".to_string();

        let url = format!("{}{}{}", self.server_url, self.base_path, url_tpl);

        let body = serde_json::to_vec_pretty(&req.body)
            .map_err(|e| format!("Can't parse request body. Error: {e}"))?;
        //TODO: params: header, query

        let client = reqwest::Client::new();
        //TODO: transform ExchangeCodeRequest into reqwest object
        let res = client
            .post(url)
            .header("Accept", "application/json")
            .body(body)
            .send()
            .await;

        match res {
            Ok(response) => match response.status() {
                StatusCode::CREATED => match response.json::<ExchangeCodeResultSchema>().await {
                    Ok(data) => Ok(ExchangeCodeOperationResponseEnum::Status201(
                        CodeExchangedSuccessfullyResponse::new_application_json(data),
                    )),
                    Err(e) => Err(e.to_string()),
                },
                StatusCode::BAD_REQUEST => match response.json::<ProblemDetailSchema>().await {
                    Ok(data) => Ok(ExchangeCodeOperationResponseEnum::Status400(
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
