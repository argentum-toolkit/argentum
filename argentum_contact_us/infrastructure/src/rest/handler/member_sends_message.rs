use argentum_contact_us_business::use_case::member_sends_message::{
    MemberSendsMessageError, MemberSendsMessageUc,
};
use argentum_contact_us_rest::dto::operation_response_enum::MemberSendsMessageOperationResponseEnum;
use argentum_contact_us_rest::dto::request::MemberSendsMessageRequest;
use argentum_contact_us_rest::dto::response::MessageCreatedSuccessfullyResponse;
use argentum_contact_us_rest::dto::schema::MessageCreated;
use argentum_contact_us_rest::server::handler::MemberSendsMessageTrait;
use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::data_type::error::{
    BadRequestError, HttpError, InternalServerError, ServiceUnavailableError, Unauthorized,
};
use argentum_standard_business::invariant_violation::{ViolationItem, Violations};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::entity::user::{User, UserTrait};
use async_trait::async_trait;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::rest::transformer::DtoToMemberSendsMessageParams;

pub struct MemberSendsMessageHandler<L>
where
    L: LoggerTrait,
{
    uc: Arc<MemberSendsMessageUc<UniqueId, L>>,
    transformer: Arc<DtoToMemberSendsMessageParams>,
}

impl<L> MemberSendsMessageHandler<L>
where
    L: LoggerTrait,
{
    pub fn new(
        uc: Arc<MemberSendsMessageUc<UniqueId, L>>,
        transformer: Arc<DtoToMemberSendsMessageParams>,
    ) -> Self {
        Self { uc, transformer }
    }
}

fn captcha_field_error(captcha_violations: Violations) -> HttpError {
    HttpError::BadRequest(BadRequestError::new(
        Violations::new(
            vec![],
            Some(ViolationItem::Object(BTreeMap::from([(
                "captcha".to_string(),
                captcha_violations,
            )]))),
        ),
        Violations::new(vec![], None),
        Violations::new(vec![], None),
        Violations::new(vec![], None),
    ))
}

fn field_error(field: &str, message: &str) -> HttpError {
    HttpError::BadRequest(BadRequestError::new(
        Violations::new(
            vec![],
            Some(ViolationItem::Object(BTreeMap::from([(
                field.to_string(),
                Violations::new(vec![message.to_string()], None),
            )]))),
        ),
        Violations::new(vec![], None),
        Violations::new(vec![], None),
        Violations::new(vec![], None),
    ))
}

#[async_trait]
impl<L> MemberSendsMessageTrait for MemberSendsMessageHandler<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn handle(
        &self,
        req: MemberSendsMessageRequest,
        user: User<Self::Id>,
    ) -> Result<MemberSendsMessageOperationResponseEnum, HttpError> {
        let member = match user {
            User::Member(m) => m,
            User::Guest(_) => {
                return Err(HttpError::Unauthorized(Unauthorized::new(
                    "Endpoint is available only for members",
                )));
            }
        };

        let (captcha, subject, message) = self.transformer.transform(req)?;

        let result = self
            .uc
            .execute(captcha, member.id(), subject, message)
            .await;

        match result {
            Ok(request) => {
                let dto = MessageCreated::new(request.id.into());

                Ok(MemberSendsMessageOperationResponseEnum::Status201(
                    MessageCreatedSuccessfullyResponse::new_application_json(dto),
                ))
            }

            Err(e) => match e {
                MemberSendsMessageError::CaptchaFailed => Err(captcha_field_error(
                    Violations::new(vec!["captcha check failed".to_string()], None),
                )),
                MemberSendsMessageError::CaptchaUnavailable(source) => Err(
                    HttpError::ServiceUnavailable(ServiceUnavailableError::new(Arc::new(source))),
                ),
                MemberSendsMessageError::InvalidSubject(source) => {
                    Err(field_error("subject", &source.to_string()))
                }
                MemberSendsMessageError::InvalidMessage(source) => {
                    Err(field_error("message", &source.to_string()))
                }
                MemberSendsMessageError::SavingRequestError(_) => Err(
                    HttpError::InternalServerError(InternalServerError::new(Arc::new(e))),
                ),
            },
        }
    }
}
