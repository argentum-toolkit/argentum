use argentum_contact_us_business::use_case::guest_sends_message::{
    GuestSendsMessageError, GuestSendsMessageUc,
};
use argentum_contact_us_rest::dto::operation_response_enum::GuestSendsMessageOperationResponseEnum;
use argentum_contact_us_rest::dto::request::GuestSendsMessageRequest;
use argentum_contact_us_rest::dto::response::MessageCreatedSuccessfullyResponse;
use argentum_contact_us_rest::dto::schema::MessageCreated;
use argentum_contact_us_rest::server::handler::GuestSendsMessageTrait;
use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::data_type::error::{
    BadRequestError, HttpError, InternalServerError, ServiceUnavailableError,
};
use argentum_standard_business::invariant_violation::{ViolationItem, Violations};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use async_trait::async_trait;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::rest::transformer::DtoToGuestSendsMessageParams;

pub struct GuestSendsMessageHandler<L>
where
    L: LoggerTrait,
{
    uc: Arc<GuestSendsMessageUc<UniqueId, L>>,
    transformer: Arc<DtoToGuestSendsMessageParams>,
}

impl<L> GuestSendsMessageHandler<L>
where
    L: LoggerTrait,
{
    pub fn new(
        uc: Arc<GuestSendsMessageUc<UniqueId, L>>,
        transformer: Arc<DtoToGuestSendsMessageParams>,
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
impl<L> GuestSendsMessageTrait for GuestSendsMessageHandler<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn handle(
        &self,
        req: GuestSendsMessageRequest,
    ) -> Result<GuestSendsMessageOperationResponseEnum, HttpError> {
        let (captcha, name, email, subject, message) = self.transformer.transform(req)?;

        let result = self
            .uc
            .execute(captcha, name, email, subject, message)
            .await;

        match result {
            Ok(request) => {
                let dto = MessageCreated::new(request.id.into());

                Ok(GuestSendsMessageOperationResponseEnum::Status201(
                    MessageCreatedSuccessfullyResponse::new_application_json(dto),
                ))
            }

            Err(e) => match e {
                GuestSendsMessageError::CaptchaFailed => Err(captcha_field_error(Violations::new(
                    vec!["captcha check failed".to_string()],
                    None,
                ))),
                GuestSendsMessageError::CaptchaUnavailable(source) => Err(
                    HttpError::ServiceUnavailable(ServiceUnavailableError::new(Arc::new(source))),
                ),
                GuestSendsMessageError::InvalidSubject(source) => {
                    Err(field_error("subject", &source.to_string()))
                }
                GuestSendsMessageError::InvalidMessage(source) => {
                    Err(field_error("message", &source.to_string()))
                }
                GuestSendsMessageError::SavingRequestError(_) => Err(
                    HttpError::InternalServerError(InternalServerError::new(Arc::new(e))),
                ),
            },
        }
    }
}
