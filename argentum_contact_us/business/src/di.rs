use crate::mock::notifier::AdminNotifierMock;
use crate::mock::repository::message_repository_mock::MessageRepositoryMock;
use crate::notifier::admin_notifier::AdminNotifierTrait;
use crate::repository::message_repository::MessageRepositoryTrait;
use crate::use_case::guest_sends_message::GuestSendsMessageUc;
use crate::use_case::member_sends_message::MemberSendsMessageUc;
use argentum_captcha_business::use_case::UserPassesCaptchaUcTrait;
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_standard_business::di_container::{Container, DiError};
use std::sync::Arc;

pub fn contact_us_business_provide<I, L>(container: &mut Container)
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    container.provide_factory::<GuestSendsMessageUc<I, L>, _>(|c| {
        let id_factory = c.resolve::<dyn IdFactory<Id = I>>()?;
        let message_repository = c.resolve::<dyn MessageRepositoryTrait<Id = I>>()?;
        let admin_notifier = c.resolve::<dyn AdminNotifierTrait<Id = I>>()?;
        let captcha_uc = c.resolve::<dyn UserPassesCaptchaUcTrait>()?;
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(GuestSendsMessageUc::new(
            id_factory,
            message_repository,
            admin_notifier,
            captcha_uc,
            logger,
        )))
    });

    container.provide_factory::<MemberSendsMessageUc<I, L>, _>(|c| {
        let id_factory = c.resolve::<dyn IdFactory<Id = I>>()?;
        let message_repository = c.resolve::<dyn MessageRepositoryTrait<Id = I>>()?;
        let admin_notifier = c.resolve::<dyn AdminNotifierTrait<Id = I>>()?;
        let captcha_uc = c.resolve::<dyn UserPassesCaptchaUcTrait>()?;
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(MemberSendsMessageUc::new(
            id_factory,
            message_repository,
            admin_notifier,
            captcha_uc,
            logger,
        )))
    });
}

pub fn contact_us_business_provide_mock<I>(container: &mut Container)
where
    I: Id + 'static,
{
    container.provide_factory::<dyn MessageRepositoryTrait<Id = I>, _>(|_c| {
        Ok(Arc::new(MessageRepositoryMock::new()))
    });

    container.provide_factory::<dyn AdminNotifierTrait<Id = I>, _>(|_c| {
        Ok(Arc::new(AdminNotifierMock::<I>::passing()))
    });
}

pub type ContactUsBusiness<I, L> = (
    Arc<GuestSendsMessageUc<I, L>>,
    Arc<MemberSendsMessageUc<I, L>>,
);

pub fn resolve_contact_us_business<I, L>(
    container: &Container,
) -> Result<ContactUsBusiness<I, L>, DiError>
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    let guest_uc = container.resolve::<GuestSendsMessageUc<I, L>>()?;
    let member_uc = container.resolve::<MemberSendsMessageUc<I, L>>()?;

    Ok((guest_uc, member_uc))
}

#[cfg(test)]
mod tests {
    use argentum_captcha_business::use_case::UserPassesCaptchaUcTrait;
    use argentum_log_business::{DefaultLogger, Level, StdoutWriter};
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::di_container::Container;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use std::sync::Arc;

    use crate::mock::UserPassesCaptchaMock;

    use super::{
        contact_us_business_provide, contact_us_business_provide_mock, resolve_contact_us_business,
    };

    type SilentLogger = DefaultLogger<StdoutWriter>;

    fn silent_logger() -> Arc<SilentLogger> {
        Arc::new(DefaultLogger::new(
            Level::Error,
            Arc::new(StdoutWriter::new()),
        ))
    }

    #[tokio::test]
    async fn resolves_use_cases_wired_with_mocks() -> Result<(), String> {
        let mut container = Container::new();
        container.provide_factory::<SilentLogger, _>(move |_c| Ok(silent_logger()));
        container
            .provide_factory::<dyn IdFactory<Id = u64>, _>(|_c| Ok(Arc::new(IdFactoryMock::new())));
        container.provide_factory::<dyn UserPassesCaptchaUcTrait, _>(|_c| {
            Ok(Arc::new(UserPassesCaptchaMock::passing()))
        });

        contact_us_business_provide_mock::<u64>(&mut container);
        contact_us_business_provide::<u64, SilentLogger>(&mut container);

        let (guest_uc, member_uc) = resolve_contact_us_business::<u64, SilentLogger>(&container)
            .map_err(|error| format!("{error:?}"))?;

        let guest_request = guest_uc
            .execute(
                None,
                "John Cooper".to_string(),
                argentum_standard_business::data_type::EmailAddress::try_new("demo@test.com")
                    .map_err(|error| format!("{error:?}"))?,
                "Question".to_string(),
                "How are you?".to_string(),
            )
            .await
            .map_err(|error| format!("{error:?}"))?;

        let member_request = member_uc
            .execute(
                None,
                guest_request.id,
                "Question".to_string(),
                "How are you?".to_string(),
            )
            .await
            .map_err(|error| format!("{error:?}"))?;

        assert_eq!(member_request.user_id, Some(guest_request.id));

        Ok(())
    }
}
