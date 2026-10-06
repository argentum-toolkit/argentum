use crate::mock::repository::email_confirmation_token_repository_mock::EmailConfirmationTokenRepositoryMock;
use crate::mock::repository::restore_password_token_repository_mock::RestorePasswordTokenRepositoryMock;
use crate::mock::token::GuestGeneratorMock;
use crate::mock::token::MemberGeneratorMock;
use crate::repository::email_confirmation_token_repository::EmailConfirmationTokenRepositoryTrait;
use crate::repository::password_credential_checker::PasswordCredentialChecker;
use crate::repository::password_credential_repository::PasswordCredentialRepositoryTrait;
use crate::repository::password_credential_writer::{
    PasswordCredentialWriter, PasswordCredentialWriterTrait,
};
use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
use crate::service::{
    DefaultPasswordToTokenExchanger, EmailConfirmationInitiator, PasswordToTokenExchangerTrait,
};
use crate::token::{GuestGeneratorTrait, MemberGeneratorTrait};
use crate::use_case::guest_registers::GuestRegistersUc;
use crate::use_case::member_confirms_email::MemberConfirmsEmailUcTrait;
use crate::use_case::member_logins_with_password::MemberLoginsWithPasswordUc;
use crate::use_case::member_registers_with_password::MemberRegistersWithPasswordUc;
use crate::use_case::restore_password::guest_requests_restore_token::GuestRequestsRestoreTokenUc;
use crate::use_case::restore_password::guest_with_token_changes_password::GuestWithTokenChangesPasswordUc;
use crate::{
    mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock,
    use_case::member_confirms_email::DefaultMemberConfirmsEmailUc,
};
use argentum_encryption_business::mock::password::{EncryptorMock, ValidatorMock};
use argentum_encryption_business::password::Validator;
use argentum_event_business::di::EventBusinessDiBuilder;
use argentum_log_business::LoggerTrait;
use argentum_notification_business::NotifierTrait;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_standard_business::di_container::Container;
use argentum_standard_business::error::DiError;
use argentum_user_business::di::UserBusinessDiBuilder;
use argentum_user_business::repository::session_repository::SessionRepositoryTrait;
use argentum_user_business::repository::user_repository::{
    GuestUserRepositoryTrait, MemberRepositoryTrait,
};
use argentum_user_business::service::GuestBinder;
use std::sync::{Arc, Mutex};

pub struct UserAccountBusinessDi<I, L>
where
    I: Id + 'static,
    L: LoggerTrait,
{
    // Public services
    pub guest_registers_uc: Arc<GuestRegistersUc<I>>,
    pub member_registers_with_password_uc: Arc<MemberRegistersWithPasswordUc<I>>,
    pub member_logins_with_password_uc: Arc<MemberLoginsWithPasswordUc<I, L>>,
    pub guest_with_token_changes_password_uc: Arc<GuestWithTokenChangesPasswordUc<I, L>>,
    pub guest_requests_restore_token_uc: Arc<GuestRequestsRestoreTokenUc<I, L>>,
    pub member_confirms_email_uc: Arc<dyn MemberConfirmsEmailUcTrait<Id = I>>,
    pub member_token_generator: Arc<dyn MemberGeneratorTrait<Id = I>>,
}

pub struct UserAccountBusinessDiBuilder<I, L>
where
    I: Id + 'static,
    L: LoggerTrait,
{
    user_business_builder: Arc<Mutex<UserBusinessDiBuilder<I, L>>>,
    event_business_builder: Arc<EventBusinessDiBuilder>,

    id_factory: Arc<dyn IdFactory<Id = I>>,
    logger: Arc<L>,
    notifier: Arc<dyn NotifierTrait<Id = I>>,

    password_credential_repository: Option<Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>>,
    password_to_token_exchanger: Option<Arc<dyn PasswordToTokenExchangerTrait<Id = I>>>,
    restore_password_token_repository: Option<Arc<dyn RestorePasswordTokenRepositoryTrait<Id = I>>>,
    email_confirmation_token_repository:
        Option<Arc<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>>,
    guest_token_generator: Option<Arc<dyn GuestGeneratorTrait<Id = I>>>,
    member_token_generator: Option<Arc<dyn MemberGeneratorTrait<Id = I>>>,

    member_confirms_email_uc: Option<Arc<dyn MemberConfirmsEmailUcTrait<Id = I>>>,
    email_confirmation_token_ttl: u32,
    restore_password_token_ttl: u32,
    product_name: String,
    email_confirmation_front_url: String,
    restore_password_front_url: String,
    validator: Arc<dyn Validator>,

    built_di: Option<Arc<UserAccountBusinessDi<I, L>>>,
}

impl<I, L> UserAccountBusinessDiBuilder<I, L>
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    pub fn new(
        user_business_builder: Arc<Mutex<UserBusinessDiBuilder<I, L>>>,
        event_business_builder: Arc<EventBusinessDiBuilder>,

        product_name: &str,
        email_confirmation_token_ttl: u32,
        restore_password_token_ttl: u32,
        email_confirmation_front_url: &str,
        restore_password_front_url: &str,
        id_factory: Arc<dyn IdFactory<Id = I>>,
        logger: Arc<L>,
        notifier: Arc<dyn NotifierTrait<Id = I>>,
        validator: Arc<dyn Validator>,
    ) -> Self {
        Self {
            user_business_builder,
            event_business_builder,

            id_factory,
            logger,
            notifier,
            password_credential_repository: None,
            password_to_token_exchanger: None,
            restore_password_token_repository: None,
            email_confirmation_token_repository: None,
            guest_token_generator: None,
            member_token_generator: None,
            member_confirms_email_uc: None,
            email_confirmation_token_ttl,
            restore_password_token_ttl,
            product_name: product_name.into(),
            email_confirmation_front_url: email_confirmation_front_url.into(),
            restore_password_front_url: restore_password_front_url.into(),
            validator,
            built_di: None,
        }
    }

    pub fn mock(&mut self) -> &mut Self {
        self.password_credential_repository = Some(Arc::new(
            PasswordCredentialRepositoryMock::new(Arc::new(EncryptorMock::new())),
        ));
        self.restore_password_token_repository =
            Some(Arc::new(RestorePasswordTokenRepositoryMock::new()));

        self.email_confirmation_token_repository =
            Some(Arc::new(EmailConfirmationTokenRepositoryMock::new()));

        self.guest_token_generator = Some(Arc::new(GuestGeneratorMock::new()));
        self.member_token_generator = Some(Arc::new(MemberGeneratorMock::new()));

        self
    }

    pub fn with_default_services(
        &mut self,
        password_credential_repository: Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>,
        restore_password_token_repository: Arc<dyn RestorePasswordTokenRepositoryTrait<Id = I>>,
        email_confirmation_token_repository: Arc<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>,
        guest_token_generator: Arc<dyn GuestGeneratorTrait<Id = I>>,
        member_token_generator: Arc<dyn MemberGeneratorTrait<Id = I>>,
    ) -> &mut Self {
        self.password_credential_repository = Some(password_credential_repository);
        self.restore_password_token_repository = Some(restore_password_token_repository);
        self.email_confirmation_token_repository = Some(email_confirmation_token_repository);
        self.guest_token_generator = Some(guest_token_generator);
        self.member_token_generator = Some(member_token_generator);

        self
    }

    pub fn with_password_credential_repository(
        &mut self,
        password_credential_repository: Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>,
    ) -> &mut Self {
        self.password_credential_repository = Some(password_credential_repository);

        self
    }

    pub fn with_password_to_token_exchanger(
        &mut self,
        password_to_token_exchanger: Arc<dyn PasswordToTokenExchangerTrait<Id = I>>,
    ) -> &mut Self {
        self.password_to_token_exchanger = Some(password_to_token_exchanger);

        self
    }

    pub fn with_email_confirmation_token_repository(
        &mut self,
        email_confirmation_token_repository: Arc<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>,
    ) -> &mut Self {
        self.email_confirmation_token_repository = Some(email_confirmation_token_repository);

        self
    }

    pub fn with_member_confirms_email_uc(
        &mut self,
        member_confirms_email_uc: Arc<dyn MemberConfirmsEmailUcTrait<Id = I>>,
    ) -> &mut Self {
        self.member_confirms_email_uc = Some(member_confirms_email_uc);

        self
    }

    pub fn build(&mut self) -> Result<Arc<UserAccountBusinessDi<I, L>>, DiError> {
        if let Some(di) = &self.built_di {
            return Ok(di.clone());
        }

        let guest_token_generator = self
            .guest_token_generator
            .clone()
            .ok_or("guest_token_generator is not initialized")?;

        let member_token_generator = self
            .member_token_generator
            .clone()
            .ok_or("member_token_generator is not initialized")?;

        let password_credential_repository = self
            .password_credential_repository
            .clone()
            .ok_or("password_credential_repository is not initialized")?;

        let user_business_di = self.user_business_builder.lock()?.build()?;
        let password_to_token_exchanger = match self.password_to_token_exchanger.clone() {
            Some(p) => p,
            None => {
                let password_credential_checker = Arc::new(PasswordCredentialChecker::new(
                    password_credential_repository.clone(),
                    self.validator.clone(),
                ));

                Arc::new(DefaultPasswordToTokenExchanger::new(
                    user_business_di.member_repository.clone(),
                    user_business_di.session_repository.clone(),
                    password_credential_checker,
                    self.id_factory.clone(),
                    member_token_generator.clone(),
                ))
            }
        };

        let restore_password_token_repository = self
            .restore_password_token_repository
            .clone()
            .ok_or("restore_password_token_repository is not initialized")?;

        let email_confirmation_token_repository = self
            .email_confirmation_token_repository
            .clone()
            .ok_or("email_confirmation_token_repository is not initialized")?;

        let event_business_di = self.event_business_builder.build()?;

        let member_confirms_email_uc = match self.member_confirms_email_uc.clone() {
            Some(uc) => uc,
            None => Arc::new(DefaultMemberConfirmsEmailUc::new(
                user_business_di.member_repository.clone(),
                email_confirmation_token_repository.clone(),
                event_business_di.event_dispatcher.clone(),
                self.logger.clone(),
                self.email_confirmation_token_ttl,
            )),
        };

        let guest_registers_uc = Arc::new(GuestRegistersUc::new(
            self.id_factory.clone(),
            user_business_di.guest_repository.clone(),
            user_business_di.session_repository.clone(),
            guest_token_generator.clone(),
        ));

        let password_credential_writer = Arc::new(PasswordCredentialWriter::new(
            password_credential_repository.clone(),
        ));

        let email_confirmation_initiator =
            Arc::new(crate::service::EmailConfirmationInitiator::new(
                member_token_generator.clone(),
                email_confirmation_token_repository,
                self.notifier.clone(),
                &self.product_name,
                &self.email_confirmation_front_url,
            ));

        let user_registers_with_password_uc = Arc::new(MemberRegistersWithPasswordUc::new(
            user_business_di.member_repository.clone(),
            password_credential_writer.clone(),
            email_confirmation_initiator,
        ));

        let user_logins_with_password_uc = Arc::new(MemberLoginsWithPasswordUc::new(
            password_to_token_exchanger,
            user_business_di.guest_binder.clone(),
        ));

        let guest_with_token_changes_password_uc = Arc::new(GuestWithTokenChangesPasswordUc::new(
            user_business_di.member_repository.clone(),
            restore_password_token_repository.clone(),
            password_credential_writer,
            self.restore_password_token_ttl,
            self.logger.clone(),
        ));

        let guest_requests_restore_token_uc = Arc::new(GuestRequestsRestoreTokenUc::new(
            &self.product_name,
            &self.restore_password_front_url.clone(),
            self.id_factory.clone(),
            user_business_di.member_repository.clone(),
            restore_password_token_repository,
            guest_token_generator,
            self.notifier.clone(),
            self.logger.clone(),
        ));

        let di = Arc::new(UserAccountBusinessDi {
            guest_registers_uc,
            member_registers_with_password_uc: user_registers_with_password_uc,
            member_logins_with_password_uc: user_logins_with_password_uc,
            member_confirms_email_uc,
            guest_with_token_changes_password_uc,
            guest_requests_restore_token_uc,
            member_token_generator,
        });

        self.built_di = Some(di.clone());

        Ok(di)
    }
}

pub fn user_account_business_provide<I, L>(
    container: &mut Container,
    product_name: &'static str,
    email_confirmation_token_ttl: u32,
    restore_password_token_ttl: u32,
    email_confirmation_front_url: &'static str,
    restore_password_front_url: &'static str,
) where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    container.provide_factory::<GuestRegistersUc<_>, _>(|c| {
        let id_factory = c.resolve::<dyn IdFactory<Id = I>>()?;

        let guest_repository = c.resolve::<dyn GuestUserRepositoryTrait<Id = I>>()?;
        let session_repository = c.resolve::<dyn SessionRepositoryTrait<Id = I>>()?;
        let guest_token_generator = c.resolve::<dyn GuestGeneratorTrait<Id = I>>()?;

        Ok(Arc::new(GuestRegistersUc::new(
            id_factory,
            guest_repository,
            session_repository,
            guest_token_generator,
        )))
    });

    container.provide_factory::<MemberLoginsWithPasswordUc<_, _>, _>(|c| {
        let password_to_token_exchanger =
            c.resolve::<dyn PasswordToTokenExchangerTrait<Id = I>>()?;
        let guest_binder = c.resolve::<GuestBinder<I, L>>()?;

        Ok(Arc::new(MemberLoginsWithPasswordUc::new(
            password_to_token_exchanger,
            guest_binder,
        )))
    });

    container.provide_factory::<MemberRegistersWithPasswordUc<_>, _>(|c| {
        let member_repository = c.resolve::<dyn MemberRepositoryTrait<Id = I>>()?;
        let credential_writer = c.resolve::<dyn PasswordCredentialWriterTrait<Id = I>>()?;
        let email_confirmation_initiator = c.resolve::<EmailConfirmationInitiator<I>>()?;

        Ok(Arc::new(MemberRegistersWithPasswordUc::new(
            member_repository,
            credential_writer,
            email_confirmation_initiator,
        )))
    });

    container.provide_factory::<dyn PasswordToTokenExchangerTrait<Id = I>, _>(|c| {
        let member_repository = c.resolve::<dyn MemberRepositoryTrait<Id = I>>()?;
        let session_repository = c.resolve::<dyn SessionRepositoryTrait<Id = I>>()?;
        let password_credential_checker = c.resolve::<PasswordCredentialChecker<I>>()?;
        let id_factory = c.resolve::<dyn IdFactory<Id = I>>()?;
        let member_token_generator = c.resolve::<dyn MemberGeneratorTrait<Id = I>>()?;

        Ok(Arc::new(DefaultPasswordToTokenExchanger::new(
            member_repository,
            session_repository,
            password_credential_checker,
            id_factory,
            member_token_generator,
        )))
    });

    container.provide_factory::<dyn PasswordCredentialWriterTrait<Id = I>, _>(|c| {
        let repository = c.resolve::<dyn PasswordCredentialRepositoryTrait<Id = I>>()?;

        Ok(Arc::new(PasswordCredentialWriter::new(repository)))
    });

    container.provide_factory::<EmailConfirmationInitiator<I>, _>(|c| {
        let member_token_generator = c.resolve::<dyn MemberGeneratorTrait<Id = I>>()?;
        let email_confirmation_token_repository =
            c.resolve::<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>()?;

        let notifier = c.resolve::<dyn NotifierTrait<Id = I>>()?;

        Ok(Arc::new(EmailConfirmationInitiator::new(
            member_token_generator,
            email_confirmation_token_repository,
            notifier,
            product_name,
            email_confirmation_front_url,
        )))
    });

    container.provide_factory::<PasswordCredentialChecker<I>, _>(|c| {
        let repository = c.resolve::<dyn PasswordCredentialRepositoryTrait<Id = I>>()?;
        let validator = c.resolve::<dyn Validator>()?;

        Ok(Arc::new(PasswordCredentialChecker::new(
            repository, validator,
        )))
    });
}

pub fn user_account_business_provide_mock<I>(container: &mut Container)
where
    I: Id + 'static,
{
    container.provide_factory::<dyn MemberGeneratorTrait<Id = I>, _>(|_c| {
        Ok(Arc::new(MemberGeneratorMock::new()))
    });

    container.provide_factory::<dyn GuestGeneratorTrait<Id = I>, _>(|_c| {
        Ok(Arc::new(GuestGeneratorMock::new()))
    });

    container.provide_factory::<dyn Validator, _>(|_c| Ok(Arc::new(ValidatorMock::new())));

    container.provide_factory::<dyn PasswordCredentialRepositoryTrait<Id = I>, _>(|_c| {
        Ok(Arc::new(PasswordCredentialRepositoryMock::new(Arc::new(
            EncryptorMock::new(),
        ))))
    });

    container.provide_factory::<dyn EmailConfirmationTokenRepositoryTrait<Id = I>, _>(|_c| {
        Ok(Arc::new(EmailConfirmationTokenRepositoryMock::new()))
    });
}
