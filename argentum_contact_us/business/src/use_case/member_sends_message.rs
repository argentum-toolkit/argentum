use crate::entity::Message;
use crate::entity::MessageError;
use crate::notifier::admin_notifier::AdminNotifierTrait;
use crate::repository::message_repository::MessageRepositoryError;
use crate::repository::message_repository::MessageRepositoryTrait;

use argentum_captcha_business::{
    CaptchaError, data_type::Captcha, use_case::UserPassesCaptchaUcTrait,
};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use std::sync::Arc;

pub struct MemberSendsMessageUc<I, L>
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

impl<I, L> MemberSendsMessageUc<I, L>
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
        member_id: I,
        subject: String,
        message: String,
    ) -> Result<Message<I>, MemberSendsMessageError> {
        if !self.captcha_uc.execute(captcha.as_ref()).await? {
            return Err(MemberSendsMessageError::CaptchaFailed);
        }

        let message = Message::new_member(self.id_factory.create(), member_id, &subject, &message)
            .map_err(|e| match e {
                MessageError::EmptyMessage => MemberSendsMessageError::InvalidMessage(e),
                _ => MemberSendsMessageError::InvalidSubject(e),
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
pub enum MemberSendsMessageError {
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
    use crate::use_case::member_sends_message::{MemberSendsMessageError, MemberSendsMessageUc};
    use argentum_captcha_business::data_type::Captcha;
    use argentum_captcha_business::use_case::UserPassesCaptchaUcTrait;
    use argentum_log_business::{DefaultLogger, Level, StdoutWriter};
    use argentum_standard_business::data_type::id::IdFactory;
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

    fn member_uc(
        captcha_uc: Arc<dyn UserPassesCaptchaUcTrait>,
        writer: Arc<dyn MessageRepositoryTrait<Id = u64>>,
        notifier: Arc<dyn AdminNotifierTrait<Id = u64>>,
    ) -> MemberSendsMessageUc<u64, DefaultLogger<StdoutWriter>> {
        MemberSendsMessageUc::new(
            Arc::new(IdFactoryMock::new()),
            writer,
            notifier,
            captcha_uc,
            logger(),
        )
    }

    #[tokio::test]
    async fn member_sends_message() -> Result<(), String> {
        let repository = Arc::new(MessageRepositoryMock::new());
        let notifier = Arc::new(AdminNotifierMock::passing());
        let uc = member_uc(
            Arc::new(UserPassesCaptchaMock::passing()),
            repository.clone(),
            notifier.clone(),
        );

        let member_id = IdFactoryMock::new().create();
        let result = uc
            .execute(
                Some(token()),
                member_id,
                "Question".to_string(),
                "How are you?".to_string(),
            )
            .await;

        let request = result.map_err(|e| format!("{e:?}"))?;
        assert_eq!(request.user_id, Some(member_id));
        assert!(request.name.is_none());
        assert!(request.email.is_none());

        assert_eq!(repository.len(), 1);

        let sent = notifier.sent();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].subject, "Question");
        assert_eq!(sent[0].user_id, Some(member_id));

        Ok(())
    }

    #[tokio::test]
    async fn broken_repository_stops_member_request() -> Result<(), String> {
        let uc = member_uc(
            Arc::new(UserPassesCaptchaMock::passing()),
            Arc::new(MessageRepositoryMockWithBrokenSave::new()),
            Arc::new(AdminNotifierMock::passing()),
        );

        let member_id = IdFactoryMock::new().create();
        let result = uc
            .execute(
                Some(token()),
                member_id,
                "Question".to_string(),
                "How are you?".to_string(),
            )
            .await;

        assert!(matches!(
            result,
            Err(MemberSendsMessageError::SavingRequestError(_))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn captcha_failure_rejects_member_request_without_saving() -> Result<(), String> {
        let repository = Arc::new(MessageRepositoryMock::new());
        let uc = member_uc(
            Arc::new(UserPassesCaptchaMock::failing()),
            repository.clone(),
            Arc::new(AdminNotifierMock::passing()),
        );

        let member_id = IdFactoryMock::new().create();
        let result = uc
            .execute(
                Some(token()),
                member_id,
                "Question".to_string(),
                "How are you?".to_string(),
            )
            .await;

        assert!(matches!(
            result,
            Err(MemberSendsMessageError::CaptchaFailed)
        ));
        assert!(repository.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn unavailable_captcha_provider_stops_member_request() -> Result<(), String> {
        let repository = Arc::new(MessageRepositoryMock::new());
        let uc = member_uc(
            Arc::new(UserPassesCaptchaMock::unavailable()),
            repository.clone(),
            Arc::new(AdminNotifierMock::passing()),
        );

        let member_id = IdFactoryMock::new().create();
        let result = uc
            .execute(
                Some(token()),
                member_id,
                "Question".to_string(),
                "How are you?".to_string(),
            )
            .await;

        assert!(matches!(
            result,
            Err(MemberSendsMessageError::CaptchaUnavailable(_))
        ));
        assert!(repository.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn failed_notification_does_not_fail_member_request() -> Result<(), String> {
        let repository = Arc::new(MessageRepositoryMock::new());
        let notifier = Arc::new(AdminNotifierMock::failing());
        let uc = member_uc(
            Arc::new(UserPassesCaptchaMock::passing()),
            repository.clone(),
            notifier.clone(),
        );

        let member_id = IdFactoryMock::new().create();
        let result = uc
            .execute(
                Some(token()),
                member_id,
                "Question".to_string(),
                "How are you?".to_string(),
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(repository.len(), 1);
        assert_eq!(notifier.sent().len(), 1);
        Ok(())
    }
}
