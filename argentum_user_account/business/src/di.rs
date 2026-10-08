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
use crate::specification::StrongPasswordSpecification;
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
use argentum_event_business::EventDispatcher;
use argentum_log_business::LoggerTrait;
use argentum_notification_business::NotifierTrait;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_standard_business::di_container::Container;

use argentum_user_business::repository::session_repository::SessionRepositoryTrait;
use argentum_user_business::repository::user_repository::{
    GuestUserRepositoryTrait, MemberRepositoryTrait,
};
use argentum_user_business::service::GuestBinder;
use std::sync::Arc;

pub fn user_account_business_provide<I, L>(
    container: &mut Container,
    product_name: impl Into<String>,
    email_confirmation_token_ttl: u32,
    restore_password_token_ttl: u32,
    email_confirmation_front_url: impl Into<String>,
    restore_password_front_url: impl Into<String>,
    password_min_length: usize,
    password_complexity_enabled: bool,
) where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    let product_name = product_name.into();
    let email_confirmation_front_url = email_confirmation_front_url.into();
    let restore_password_front_url = restore_password_front_url.into();

    container.provide_factory::<StrongPasswordSpecification, _>(move |_c| {
        Ok(Arc::new(StrongPasswordSpecification::new(
            password_min_length,
            password_complexity_enabled,
        )))
    });

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
        let password_specification = c.resolve::<StrongPasswordSpecification>()?;

        Ok(Arc::new(MemberRegistersWithPasswordUc::new(
            member_repository,
            credential_writer,
            email_confirmation_initiator,
            password_specification,
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

    let email_product_name = product_name.clone();
    let email_confirmation_url = email_confirmation_front_url.clone();
    container.provide_factory::<EmailConfirmationInitiator<I>, _>(move |c| {
        let member_token_generator = c.resolve::<dyn MemberGeneratorTrait<Id = I>>()?;
        let email_confirmation_token_repository =
            c.resolve::<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>()?;

        let notifier = c.resolve::<dyn NotifierTrait<Id = I>>()?;

        Ok(Arc::new(EmailConfirmationInitiator::new(
            member_token_generator,
            email_confirmation_token_repository,
            notifier,
            &email_product_name,
            &email_confirmation_url,
        )))
    });

    container.provide_factory::<PasswordCredentialChecker<I>, _>(|c| {
        let repository = c.resolve::<dyn PasswordCredentialRepositoryTrait<Id = I>>()?;
        let validator = c.resolve::<dyn Validator>()?;

        Ok(Arc::new(PasswordCredentialChecker::new(
            repository, validator,
        )))
    });

    container.provide_factory::<dyn MemberConfirmsEmailUcTrait<Id = I>, _>(move |c| {
        let member_repository = c.resolve::<dyn MemberRepositoryTrait<Id = I>>()?;
        let email_confirmation_token_repository =
            c.resolve::<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>()?;
        let event_dispatcher = c.resolve::<EventDispatcher>()?;
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(DefaultMemberConfirmsEmailUc::new(
            member_repository,
            email_confirmation_token_repository,
            event_dispatcher,
            logger,
            email_confirmation_token_ttl,
        )))
    });

    container.provide_factory::<GuestWithTokenChangesPasswordUc<I, L>, _>(move |c| {
        let member_repository = c.resolve::<dyn MemberRepositoryTrait<Id = I>>()?;
        let restore_password_token_repository =
            c.resolve::<dyn RestorePasswordTokenRepositoryTrait<Id = I>>()?;
        let credential_writer = c.resolve::<dyn PasswordCredentialWriterTrait<Id = I>>()?;
        let password_specification = c.resolve::<StrongPasswordSpecification>()?;
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(GuestWithTokenChangesPasswordUc::new(
            member_repository,
            restore_password_token_repository,
            credential_writer,
            password_specification,
            restore_password_token_ttl,
            logger,
        )))
    });

    container.provide_factory::<GuestRequestsRestoreTokenUc<I, L>, _>(move |c| {
        let id_factory = c.resolve::<dyn IdFactory<Id = I>>()?;
        let member_repository = c.resolve::<dyn MemberRepositoryTrait<Id = I>>()?;
        let restore_password_token_repository =
            c.resolve::<dyn RestorePasswordTokenRepositoryTrait<Id = I>>()?;
        let guest_token_generator = c.resolve::<dyn GuestGeneratorTrait<Id = I>>()?;
        let notifier = c.resolve::<dyn NotifierTrait<Id = I>>()?;
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(GuestRequestsRestoreTokenUc::new(
            &product_name,
            &restore_password_front_url,
            id_factory,
            member_repository,
            restore_password_token_repository,
            guest_token_generator,
            notifier,
            logger,
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

    container.provide_factory::<dyn RestorePasswordTokenRepositoryTrait<Id = I>, _>(|_c| {
        Ok(Arc::new(RestorePasswordTokenRepositoryMock::new()))
    });
}
