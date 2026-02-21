use crate::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
use crate::mock::repository::restore_password_token_repository_mock::RestorePasswordTokenRepositoryMock;
use crate::mock::token::TokenGeneratorMock;
use crate::repository::password_credential_repository::PasswordCredentialRepositoryTrait;
use crate::repository::password_credential_writer::PasswordCredentialWriter;
use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
use crate::service::PasswordToTokenExchangerTrait;
use crate::token::GeneratorTrait;
use crate::use_case::anonymous_registers::AnonymousRegistersUc;
use crate::use_case::member_confirms_email::MemberConfirmsEmailUcTrait;
use crate::use_case::restore_password::anonymous_requests_restore_token::AnonymousRequestsRestoreTokenUc;
use crate::use_case::restore_password::anonymous_with_token_changes_password::AnonymousWithTokenChangesPasswordUc;
use crate::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
use crate::use_case::user_registers_with_password::UserRegistersWithPasswordUc;
use argentum_encryption_business::mock::password::EncryptorMock;
use argentum_log_business::LoggerTrait;
use argentum_notification_business::NotificatorTrait;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_user_business::mock::repository::anonymous_user_repository_mock::AnonymousUserRepositoryMock;
use argentum_user_business::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
use argentum_user_business::mock::repository::session_repository_mock::SessionRepositoryMock;
use argentum_user_business::repository::session_repository::SessionRepositoryTrait;
use argentum_user_business::repository::user_repository::{
    AnonymousUserRepositoryTrait, AuthenticatedUserRepositoryTrait,
};
use argentum_user_business::service::{GuestBinder, MemberAuthenticatorTrait, TokenRouterTrait};
use argentum_user_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use std::sync::Arc;

pub struct BusinessDiC<I, L>
where
    I: Id + 'static,
    L: LoggerTrait,
{
    // Public services
    pub anonymous_registers_uc: Arc<AnonymousRegistersUc<I>>,
    pub user_registers_with_password_uc: Arc<UserRegistersWithPasswordUc<I>>,
    pub user_logins_with_password_uc: Arc<UserLoginsWithPasswordUc<I, L>>,
    pub user_authenticates_with_token_uc: Arc<UserAuthenticatesWithTokenUc<I>>,
    pub anonymous_with_token_changes_password_uc: Arc<AnonymousWithTokenChangesPasswordUc<I, L>>,
    pub anonymous_requests_restore_token_uc: Arc<AnonymousRequestsRestoreTokenUc<I, L>>,
    pub member_confirms_email_uc: Arc<dyn MemberConfirmsEmailUcTrait<Id = I>>,
    pub member_token_generator: Arc<dyn GeneratorTrait<Id = I>>,
}

pub struct UserAccountBusinessDiCBuilder<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    id_factory: Arc<dyn IdFactory<Id = I>>,
    // validator: Arc<dyn Validator>,
    logger: Arc<L>,
    notificator: Arc<dyn NotificatorTrait<Id = I>>,

    guest_binder: Option<Arc<GuestBinder<I, L>>>,
    anonymous_user_repository: Option<Arc<dyn AnonymousUserRepositoryTrait<Id = I>>>,
    authenticated_user_repository: Option<Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>>,
    session_repository: Option<Arc<dyn SessionRepositoryTrait<Id = I>>>,
    password_credential_repository: Option<Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>>,
    password_to_token_exchanger: Option<Arc<dyn PasswordToTokenExchangerTrait<Id = I>>>,
    restore_password_token_repository: Option<Arc<dyn RestorePasswordTokenRepositoryTrait<Id = I>>>,
    guest_token_generator: Option<Arc<dyn GeneratorTrait<Id = I>>>,
    member_token_generator: Option<Arc<dyn GeneratorTrait<Id = I>>>,
    token_router: Option<Arc<dyn TokenRouterTrait>>,
    member_authenticator: Option<Arc<dyn MemberAuthenticatorTrait<Id = I>>>,

    member_confirms_email_uc: Option<Arc<dyn MemberConfirmsEmailUcTrait<Id = I>>>,
    restore_password_token_ttl: u32,
    product_name: String,
    restore_password_front_url: String,
}

impl<I, L> UserAccountBusinessDiCBuilder<I, L>
where
    //TODO: not static
    I: Id + 'static,
    L: LoggerTrait,
{
    pub fn new(
        id_factory: Arc<dyn IdFactory<Id = I>>,
        // validator: Arc<dyn Validator>,
        logger: Arc<L>,
        notificator: Arc<dyn NotificatorTrait<Id = I>>,
    ) -> Self {
        Self {
            id_factory,
            // validator,
            logger,
            notificator,
            guest_binder: None,
            anonymous_user_repository: None,
            authenticated_user_repository: None,
            session_repository: None,
            password_credential_repository: None,
            password_to_token_exchanger: None,
            restore_password_token_repository: None,
            guest_token_generator: None,
            member_token_generator: None,
            token_router: None,
            member_authenticator: None,
            member_confirms_email_uc: None,
            restore_password_token_ttl: 1,
            product_name: "".into(),
            restore_password_front_url: "".into(),
        }
    }

    pub fn mock(&mut self) -> &mut Self {
        // self.guest_binder = Some(Arc::new(AnonymousBindingRepositoryMock::new()));
        self.anonymous_user_repository = Some(Arc::new(AnonymousUserRepositoryMock::new()));
        self.authenticated_user_repository = Some(Arc::new(AuthenticatedUserRepositoryMock::new()));
        self.session_repository = Some(Arc::new(SessionRepositoryMock::new()));

        self.password_credential_repository = Some(Arc::new(
            PasswordCredentialRepositoryMock::new(Arc::new(EncryptorMock::new())),
        ));
        self.restore_password_token_repository =
            Some(Arc::new(RestorePasswordTokenRepositoryMock::new()));

        self.guest_token_generator = Some(Arc::new(TokenGeneratorMock::new()));
        self.member_token_generator = Some(Arc::new(TokenGeneratorMock::new()));

        self
    }

    pub fn config<S>(
        &mut self,
        product_name: S,
        restore_password_token_ttl: u32,
        restore_password_front_url: S,
    ) -> &mut Self
    where
        S: Into<String>,
    {
        self.product_name = product_name.into();
        self.restore_password_token_ttl = restore_password_token_ttl;
        self.restore_password_front_url = restore_password_front_url.into();

        self
    }

    pub fn with_authenticated_user_repository(
        &mut self,
        authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>,
    ) -> &mut Self {
        self.authenticated_user_repository = Some(authenticated_user_repository);

        self
    }

    pub fn services(
        &mut self,
        guest_binder: Arc<GuestBinder<I, L>>,
        anonymous_user_repository: Arc<dyn AnonymousUserRepositoryTrait<Id = I>>,
        authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>,
        session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
        password_credential_repository: Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>,
        password_to_token_exchanger: Arc<dyn PasswordToTokenExchangerTrait<Id = I>>,
        restore_password_token_repository: Arc<dyn RestorePasswordTokenRepositoryTrait<Id = I>>,
        guest_token_generator: Arc<dyn GeneratorTrait<Id = I>>,
        member_token_generator: Arc<dyn GeneratorTrait<Id = I>>,
        token_router: Arc<dyn TokenRouterTrait>,
        member_authenticator: Arc<dyn MemberAuthenticatorTrait<Id = I>>,
        member_confirms_email_uc: Arc<dyn MemberConfirmsEmailUcTrait<Id = I>>,
    ) -> &mut Self {
        self.guest_binder = Some(guest_binder);
        self.anonymous_user_repository = Some(anonymous_user_repository);
        self.authenticated_user_repository = Some(authenticated_user_repository);
        self.session_repository = Some(session_repository);
        self.password_credential_repository = Some(password_credential_repository);
        self.password_to_token_exchanger = Some(password_to_token_exchanger);
        self.restore_password_token_repository = Some(restore_password_token_repository);
        self.guest_token_generator = Some(guest_token_generator);
        self.member_token_generator = Some(member_token_generator);
        self.token_router = Some(token_router);
        self.member_authenticator = Some(member_authenticator);
        self.member_confirms_email_uc = Some(member_confirms_email_uc);

        self
    }

    pub fn build(&self) -> Result<BusinessDiC<I, L>, String> {
        let anonymous_user_repository = self
            .anonymous_user_repository
            .clone()
            .ok_or("anonymous_user_repository is not initialized")?;

        let session_repository = self
            .session_repository
            .clone()
            .ok_or("session_repository is not initialized")?;

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

        let password_to_token_exchanger = self
            .password_to_token_exchanger
            .clone()
            .ok_or("password_to_token_exchanger is not initialized")?;

        let authenticated_user_repository = self
            .authenticated_user_repository
            .clone()
            .ok_or("authenticated_user_repository is not initialized")?;

        let guest_binder = self
            .guest_binder
            .clone()
            .ok_or("guest_binder is not initialized")?;

        let restore_password_token_repository = self
            .restore_password_token_repository
            .clone()
            .ok_or("restore_password_token_repository is not initialized")?;

        let token_router = self
            .token_router
            .clone()
            .ok_or("token_router not initialized")?;

        let member_authenticator = self
            .member_authenticator
            .clone()
            .ok_or("member_authenticator not initialized")?;

        let member_confirms_email_uc = self
            .member_confirms_email_uc
            .clone()
            .ok_or("member_confirms_email_uc not initialized")?;

        let anonymous_registers_uc = Arc::new(AnonymousRegistersUc::new(
            self.id_factory.clone(),
            anonymous_user_repository.clone(),
            session_repository.clone(),
            guest_token_generator.clone(),
        ));

        let password_credential_writer = Arc::new(PasswordCredentialWriter::new(
            password_credential_repository.clone(),
        ));

        let user_registers_with_password_uc = Arc::new(UserRegistersWithPasswordUc::new(
            authenticated_user_repository.clone(),
            password_credential_writer.clone(),
        ));

        // let password_credential_checker = Arc::new(PasswordCredentialChecker::new(
        //     password_credential_repository,
        //     self.validator.clone(),
        // ));

        // let password_to_token_exchanger = Arc::new(DefaultPasswordToTokenExchanger::new(
        //     authenticated_user_repository.clone(),
        //     session_repository.clone(),
        //     password_credential_checker,
        //     self.id_factory.clone(),
        //     token_generator.clone(),
        // ));

        let user_logins_with_password_uc = Arc::new(UserLoginsWithPasswordUc::new(
            password_to_token_exchanger,
            guest_binder,
        ));

        let user_authenticates_with_token_uc = Arc::new(UserAuthenticatesWithTokenUc::new(
            member_authenticator,
            anonymous_user_repository,
            session_repository,
            token_router,
        ));

        let anonymous_with_token_changes_password_uc =
            Arc::new(AnonymousWithTokenChangesPasswordUc::new(
                authenticated_user_repository.clone(),
                restore_password_token_repository.clone(),
                password_credential_writer,
                self.restore_password_token_ttl,
                self.logger.clone(),
            ));

        let anonymous_requests_restore_token_uc = Arc::new(AnonymousRequestsRestoreTokenUc::new(
            &self.product_name,
            &self.restore_password_front_url.clone(),
            self.id_factory.clone(),
            authenticated_user_repository,
            restore_password_token_repository,
            guest_token_generator,
            self.notificator.clone(),
            self.logger.clone(),
        ));

        Ok(BusinessDiC {
            anonymous_registers_uc,
            user_registers_with_password_uc,
            user_logins_with_password_uc,
            user_authenticates_with_token_uc,
            member_confirms_email_uc,
            anonymous_with_token_changes_password_uc,
            anonymous_requests_restore_token_uc,
            member_token_generator,
        })
    }
}
