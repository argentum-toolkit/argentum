use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_notification_business::NotifierTrait;
use argentum_notification_business::mock::StdoutNotifier;
use argentum_standard_business::data_type::id::{IdFactory, UIntId};
use argentum_standard_business::di_container::{Container, DiError};
use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
use argentum_user_account_business::di::{
    user_account_business_provide, user_account_business_provide_mock,
};
use argentum_user_account_business::use_case::guest_registers::GuestRegistersUc;
use argentum_user_account_business::use_case::member_logins_with_password::MemberLoginsWithPasswordUc;
use argentum_user_account_business::use_case::member_registers_with_password::MemberRegistersWithPasswordUc;
use argentum_user_business::di::{user_business_provide, user_business_provide_mock};
use argentum_user_business::mock::service::MemberAuthenticatorMock;
use argentum_user_business::service::MemberAuthenticatorTrait;
use argentum_user_business::use_case::member_authenticates_with_token::MemberAuthenticatesWithTokenUc;
use std::sync::Arc;

use crate::app::App;

pub struct Di<L>
where
    L: LoggerTrait,
{
    pub app: App<UIntId, L>,
}

impl<L> Di<L>
where
    L: LoggerTrait,
{
    pub fn new(app: App<UIntId, L>) -> Self {
        Self { app }
    }
}

pub fn di_factory() -> Result<Di<DefaultLogger<PrettyWriter>>, DiError> {
    let mut container = Container::new();

    user_business_provide_mock::<UIntId>(&mut container);
    user_business_provide::<UIntId, DefaultLogger<PrettyWriter>>(&mut container);

    user_account_business_provide::<UIntId, DefaultLogger<PrettyWriter>>(
        &mut container,
        "Argentum ToolKit. Example of CLI application",
        3600, // TTL 1h
        3600, // TTL 1h
        "http://localhost:8082/confirm_email/",
        "http://localhost:8082/change-password/",
    );
    user_account_business_provide_mock::<UIntId>(&mut container);

    container.provide_factory::<dyn MemberAuthenticatorTrait<Id = UIntId>, _>(|_c| {
        Ok(Arc::new(MemberAuthenticatorMock::new()))
    });

    register_logger(&mut container);
    register_id_factory(&mut container);
    register_notifier(&mut container);

    let guest_registers_uc: Arc<GuestRegistersUc<_>> =
        container.resolve::<GuestRegistersUc<UIntId>>().unwrap();

    let member_registers_with_pw: Arc<MemberRegistersWithPasswordUc<_>> = container
        .resolve::<MemberRegistersWithPasswordUc<UIntId>>()
        .unwrap();

    let member_logins_with_pw: Arc<MemberLoginsWithPasswordUc<_, _>> = container
        .resolve::<MemberLoginsWithPasswordUc<UIntId, DefaultLogger<PrettyWriter>>>()
        .unwrap();

    let member_authenticates_with_token: Arc<MemberAuthenticatesWithTokenUc<_>> = container
        .resolve::<MemberAuthenticatesWithTokenUc<UIntId>>()
        .unwrap();

    let id_factory: Arc<_> = container.resolve::<dyn IdFactory<Id = UIntId>>().unwrap();
    let logger: Arc<_> = container.resolve::<DefaultLogger<PrettyWriter>>().unwrap();

    let app = App::new(
        id_factory,
        guest_registers_uc,
        member_logins_with_pw,
        member_registers_with_pw,
        member_authenticates_with_token,
        logger,
    );

    Ok(Di::new(app))
}

fn register_logger(container: &mut Container) {
    container.provide_factory::<DefaultLogger<PrettyWriter>, _>(|_c| {
        let log_writer = Arc::new(PrettyWriter::new());

        Ok(Arc::new(DefaultLogger::new(Level::Trace, log_writer)))
    });
}

fn register_id_factory(container: &mut Container) {
    container
        .provide_factory::<dyn IdFactory<Id = UIntId>, _>(|_c| Ok(Arc::new(IdFactoryMock::new())));
}

fn register_notifier(container: &mut Container) {
    container.provide_factory::<dyn NotifierTrait<Id = UIntId>, _>(|_c| {
        Ok(Arc::new(StdoutNotifier::new()))
    });
}
