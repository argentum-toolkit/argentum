use crate::mock::repository::email_confirmation_token_repository_mock::EmailConfirmationTokenRepositoryMock;
use crate::mock::repository::restore_password_token_repository_mock::RestorePasswordTokenRepositoryMock;
use crate::mock::token::TokenGeneratorMock;
use crate::repository::email_confirmation_token_repository::EmailConfirmationTokenRepositoryTrait;
use crate::repository::password_credential_checker::PasswordCredentialChecker;
use crate::repository::password_credential_repository::PasswordCredentialRepositoryTrait;
use crate::repository::password_credential_writer::PasswordCredentialWriter;
use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
use crate::service::{DefaultPasswordToTokenExchanger, PasswordToTokenExchangerTrait};
use crate::token::GeneratorTrait;
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
use argentum_encryption_business::mock::password::EncryptorMock;
use argentum_encryption_business::password::Validator;
use argentum_log_business::LoggerTrait;
use argentum_notification_business::NotifierTrait;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_standard_business::error::DiError;
use argentum_user_business::di::UserBusinessDiBuilder;
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
    pub member_token_generator: Arc<dyn GeneratorTrait<Id = I>>,
}

pub struct UserAccountBusinessDiBuilder<I, L>
where
    I: Id + 'static,
    L: LoggerTrait,
{
    user_business_builder: Arc<Mutex<UserBusinessDiBuilder<I, L>>>,

    id_factory: Arc<dyn IdFactory<Id = I>>,
    logger: Arc<L>,
    notifier: Arc<dyn NotifierTrait<Id = I>>,

    password_credential_repository: Option<Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>>,
    password_to_token_exchanger: Option<Arc<dyn PasswordToTokenExchangerTrait<Id = I>>>,
    restore_password_token_repository: Option<Arc<dyn RestorePasswordTokenRepositoryTrait<Id = I>>>,
    email_confirmation_token_repository:
        Option<Arc<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>>,
    guest_token_generator: Option<Arc<dyn GeneratorTrait<Id = I>>>,
    member_token_generator: Option<Arc<dyn GeneratorTrait<Id = I>>>,

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

        self.guest_token_generator = Some(Arc::new(TokenGeneratorMock::new()));
        self.member_token_generator = Some(Arc::new(TokenGeneratorMock::new()));

        self
    }

    pub fn with_default_services(
        &mut self,
        password_credential_repository: Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>,
        restore_password_token_repository: Arc<dyn RestorePasswordTokenRepositoryTrait<Id = I>>,
        email_confirmation_token_repository: Arc<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>,
        guest_token_generator: Arc<dyn GeneratorTrait<Id = I>>,
        member_token_generator: Arc<dyn GeneratorTrait<Id = I>>,
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

        let member_confirms_email_uc = match self.member_confirms_email_uc.clone() {
            Some(uc) => uc,
            None => Arc::new(DefaultMemberConfirmsEmailUc::new(
                user_business_di.member_repository.clone(),
                email_confirmation_token_repository.clone(),
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
