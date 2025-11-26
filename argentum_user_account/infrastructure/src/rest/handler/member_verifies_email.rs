use argentum_rest_infrastructure::data_type::error::{BadRequestError, HttpError};
use argentum_standard_business::invariant_violation::Violations;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_rest::{
    dto::{
        operation_response_enum::MemberVerifiesEmailOperationResponseEnum,
        request::MemberVerifiesEmailRequest, response::EmptyOkResponse, schema::EmptyResponse,
    },
    server::handler::MemberVerifiesEmailTrait,
};
use argentum_user_business::entity::user::User;
use async_trait::async_trait;

pub struct MemberVerifiesEmailHandler {}

impl MemberVerifiesEmailHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MemberVerifiesEmailTrait for MemberVerifiesEmailHandler {
    type Id = UniqueId;

    async fn handle(
        &self,
        req: MemberVerifiesEmailRequest, //member's key
        _user: User<Self::Id>,           //guest's bearer token
    ) -> Result<MemberVerifiesEmailOperationResponseEnum, HttpError> {
        let client = reqwest::Client::new();
        let base_url = "http://service.keycloak:8080";
        let realm = "argentum-realm-dev";
        // let client_secret = "dj9cobGEQptoEk5gMvqadYHd6emeUQ2Y";
        // let client_id = "argentum-be-client-dev";

        let url = format!(
            "{}/realms/{}/login-actions/action?key={}",
            base_url, realm, req.body.key
        );

        println!("✅ Verifying email in Keycloak: {}", url);
        let resp = client.post(&url).send().await.unwrap();

        if resp.status().is_success() {
            println!("✅ Email verified in Keycloak!");
            Ok(MemberVerifiesEmailOperationResponseEnum::Status200(
                EmptyOkResponse::new_application_json(EmptyResponse::new()),
            ))
        } else {
            let text = resp.text().await.unwrap();
            Err(HttpError::BadRequest(BadRequestError::new(
                Violations::new(vec![text], None),
                Violations::new(vec![], None),
                Violations::new(vec![], None),
                Violations::new(vec![], None),
            )))
        }
    }
}
