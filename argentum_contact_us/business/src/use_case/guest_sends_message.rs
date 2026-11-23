use crate::entity::Message;
use crate::entity::MessageError;
use crate::notifier::admin_notifier::AdminNotifierTrait;
use crate::repository::message_repository::MessageRepositoryError;
use crate::repository::message_repository::MessageRepositoryTrait;

use argentum_captcha_business::{
    CaptchaError, data_type::Captcha, use_case::UserPassesCaptchaUcTrait,
};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use std::sync::Arc;

pub struct GuestSendsMessageUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    id_factory: Arc<dyn IdFactory<Id = I>>,
    message_repository: Arc<dyn MessageRepositoryTrait<Id = I>>,
    admin_notifier: Arc<dyn AdminNotifierTrait<Id = I>>,
    captcha_uc: Arc<dyn UserPassesCaptchaUcTrait>,
    logger: Arc<L>,
}

impl<I, L> GuestSendsMessageUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        id_factory: Arc<dyn IdFactory<Id = I>>,
        message_repository: Arc<dyn MessageRepositoryTrait<Id = I>>,
        admin_notifier: Arc<dyn AdminNotifierTrait<Id = I>>,
        captcha_uc: Arc<dyn UserPassesCaptchaUcTrait>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            id_factory,
            message_repository,
            admin_notifier,
            captcha_uc,
            logger,
        }
    }

    pub async fn execute(
        &self,
        captcha: Option<Captcha>,
        name: String,
        email: EmailAddress,
        subject: String,
        message: String,
    ) -> Result<Message<I>, GuestSendsMessageError> {
        if !self.captcha_uc.execute(captcha.as_ref()).await? {
            return Err(GuestSendsMessageError::CaptchaFailed);
        }

        let message = Message::new_guest(self.id_factory.create(), name, email, &subject, &message)
            .map_err(|e| match e {
                MessageError::EmptyMessage => GuestSendsMessageError::InvalidMessage(e),
                _ => GuestSendsMessageError::InvalidSubject(e),
            })?;

        self.message_repository.save(&message).await?;

        // A failed notification must not fail the request itself
        if let Err(e) = self.admin_notifier.send(&message).await {
            self.logger
                .error(format!("Can't notify admin about a contact request: {e:?}"));
        }

        Ok(message)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GuestSendsMessageError {
    #[error("Captcha check failed")]
    CaptchaFailed,

    #[error("Captcha can't be checked")]
    CaptchaUnavailable(
        #[source]
        #[from]
        CaptchaError,
    ),

    #[error("Invalid subject")]
    InvalidSubject(#[source] MessageError),

    #[error("Invalid message")]
    InvalidMessage(#[source] MessageError),

    #[error("Message can't be saved")]
    SavingRequestError(
        #[source]
        #[from]
        MessageRepositoryError,
    ),
}

#[cfg(test)]
mod test {
    use crate::mock::AdminNotifierMock;
    use crate::mock::UserPassesCaptchaMock;
    use crate::mock::repository::MessageRepositoryMock;
    use crate::mock::repository::MessageRepositoryMockWithBrokenSave;
    use crate::notifier::admin_notifier::AdminNotifierTrait;
    use crate::repository::message_repository::MessageRepositoryTrait;
    use crate::use_case::guest_sends_message::{GuestSendsMessageError, GuestSendsMessageUc};
    use argentum_captcha_business::data_type::Captcha;
    use argentum_captcha_business::use_case::UserPassesCaptchaUcTrait;
    use argentum_log_business::{DefaultLogger, Level, StdoutWriter};
    use argentum_standard_business::data_type::EmailAddress;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use std::sync::Arc;

    fn token() -> Captcha {
        Captcha::try_new("token-value").expect("captcha should be valid")
    }

    fn logger() -> Arc<DefaultLogger<StdoutWriter>> {
        Arc::new(DefaultLogger::new(
            Level::Error,
            Arc::new(StdoutWriter::new()),
        ))
    }

    fn guest_uc(
        captcha_uc: Arc<dyn UserPassesCaptchaUcTrait>,
        writer: Arc<dyn MessageRepositoryTrait<Id = u64>>,
        notifier: Arc<dyn AdminNotifierTrait<Id = u64>>,
    ) -> GuestSendsMessageUc<u64, DefaultLogger<StdoutWriter>> {
        GuestSendsMessageUc::new(
            Arc::new(IdFactoryMock::new()),
            writer,
            notifier,
            captcha_uc,
            logger(),
        )
    }

    fn guest_data() -> (String, EmailAddress, String, String) {
        (
            "John Cooper".to_string(),
            EmailAddress::try_new("demo@test.com").expect("Email should be valid"),
            "Question".to_string(),
            "How are you?".to_string(),
        )
    }

    #[tokio::test]
    async fn guest_sends_message() -> Result<(), String> {
        let repository = Arc::new(MessageRepositoryMock::new());
        let notifier = Arc::new(AdminNotifierMock::passing());
        let uc = guest_uc(
            Arc::new(UserPassesCaptchaMock::passing()),
            repository.clone(),
            notifier.clone(),
        );

        let (name, email, subject, message) = guest_data();
        let result = uc
            .execute(Some(token()), name, email, subject, message)
            .await;

        let request = result.map_err(|e| format!("{e:?}"))?;
        assert_eq!(request.user_id, None);
        assert_eq!(request.name.as_deref(), Some("John Cooper"));
        assert_eq!(request.subject, "Question");

        assert_eq!(repository.len(), 1);

        let sent = notifier.sent();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].subject, "Question");
        assert_eq!(
            sent[0].email.as_ref().map(|e| e.as_string()).as_deref(),
            Some("demo@test.com")
        );

        Ok(())
    }

    #[tokio::test]
    async fn broken_repository_stops_guest_request() -> Result<(), String> {
        let uc = guest_uc(
            Arc::new(UserPassesCaptchaMock::passing()),
            Arc::new(MessageRepositoryMockWithBrokenSave::new()),
            Arc::new(AdminNotifierMock::passing()),
        );

        let (name, email, subject, message) = guest_data();
        let result = uc
            .execute(Some(token()), name, email, subject, message)
            .await;

        assert!(matches!(
            result,
            Err(GuestSendsMessageError::SavingRequestError(_))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn captcha_failure_rejects_guest_request_without_saving() -> Result<(), String> {
        let repository = Arc::new(MessageRepositoryMock::new());
        let uc = guest_uc(
            Arc::new(UserPassesCaptchaMock::failing()),
            repository.clone(),
            Arc::new(AdminNotifierMock::passing()),
        );

        let (name, email, subject, message) = guest_data();
        let result = uc
            .execute(Some(token()), name, email, subject, message)
            .await;

        assert!(matches!(result, Err(GuestSendsMessageError::CaptchaFailed)));
        assert!(repository.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn unavailable_captcha_provider_stops_guest_request() -> Result<(), String> {
        let repository = Arc::new(MessageRepositoryMock::new());
        let uc = guest_uc(
            Arc::new(UserPassesCaptchaMock::unavailable()),
            repository.clone(),
            Arc::new(AdminNotifierMock::passing()),
        );

        let (name, email, subject, message) = guest_data();
        let result = uc
            .execute(Some(token()), name, email, subject, message)
            .await;

        assert!(matches!(
            result,
            Err(GuestSendsMessageError::CaptchaUnavailable(_))
        ));
        assert!(repository.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn failed_notification_does_not_fail_guest_request() -> Result<(), String> {
        let repository = Arc::new(MessageRepositoryMock::new());
        let notifier = Arc::new(AdminNotifierMock::failing());
        let uc = guest_uc(
            Arc::new(UserPassesCaptchaMock::passing()),
            repository.clone(),
            notifier.clone(),
        );

        let (name, email, subject, message) = guest_data();
        let result = uc
            .execute(Some(token()), name, email, subject, message)
            .await;

        assert!(result.is_ok());
        assert_eq!(repository.len(), 1);
        assert_eq!(notifier.sent().len(), 1);
        Ok(())
    }
}
