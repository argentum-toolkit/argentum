use crate::db::{
    UserAccountSqlxPostgresAdapter,
    repository::{
        EmailConfirmationTokenRepository, PasswordCredentialRepository,
        RestorePasswordTokenRepository,
    },
};
use crate::rest::handler::{
    GuestRegistersHandler, GuestRequestsRestoreTokenHandler, GuestWithTokenChangesPasswordHandler,
    MemberLoginsWithPasswordHandler, MemberRegistersWithPasswordHandler,
    MemberVerifiesEmailHandler,
};
use crate::rest::transformer::{
    DtoToGuestRequestsRestoreTokenParams, DtoToGuestWithTokenChangesPasswordParams,
    DtoToMemberLoginsWithPasswordParams, DtoToMemberRegistersWithPasswordParams,
};
use crate::token::{JwtTokenGenerator, StringTokenGenerator};
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_encryption_business::password::Encryptor;
use argentum_log_business::LoggerTrait;

use argentum_standard_business::{
    data_type::id::Id,
    di_container::{Container, DiError},
};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;

use argentum_user_account_business::repository::{
    email_confirmation_token_repository::EmailConfirmationTokenRepositoryTrait,
    password_credential_repository::PasswordCredentialRepositoryTrait,
    restore_password_token_repository::RestorePasswordTokenRepositoryTrait,
};

use argentum_user_account_business::token::{GuestGeneratorTrait, MemberGeneratorTrait};
use argentum_user_account_business::use_case::{
    guest_registers::GuestRegistersUc,
    member_confirms_email::MemberConfirmsEmailUcTrait,
    member_logins_with_password::MemberLoginsWithPasswordUc,
    member_registers_with_password::MemberRegistersWithPasswordUc,
    restore_password::{
        guest_requests_restore_token::GuestRequestsRestoreTokenUc,
        guest_with_token_changes_password::GuestWithTokenChangesPasswordUc,
    },
};
use argentum_user_account_rest::{
    server::handler::{
        GuestRegistersTrait, GuestRequestsRestoreTokenTrait, GuestWithTokenChangesPasswordTrait,
        MemberLoginsWithPasswordTrait, MemberRegistersWithPasswordTrait, MemberVerifiesEmailTrait,
    },
    user_account_api_provide,
};

use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub async fn user_account_infrastructure_provider<I, L>(
    container: &mut Container,
    connection_url: impl Into<String>,
    max_db_connections: u32,
    member_jwt_token_secret: impl Into<String>,
    member_jwt_token_ttl: i64,
) -> Result<(), DiError>
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    user_account_api_provide::<I>(container, "/api/v1");

    let connection_url = connection_url.into();
    let member_jwt_token_secret = member_jwt_token_secret.into();
    let pool = PgPoolOptions::new()
        .max_connections(max_db_connections)
        .min_connections(2)
        .connect(&connection_url)
        .await
        .map_err(|e| {
            DiError::Other(format!(
                "USER_ACCOUNT COMPONENT: Can't create PG connection pool for connection `{}`. Error: {e}",
                connection_url
            ))
        })?;

    container.provide_factory::<UserAccountSqlxPostgresAdapter<L>, _>(move |c| {
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(UserAccountSqlxPostgresAdapter(
            SqlxPostgresAdapter::new(pool.clone(), logger),
        )))
    });

    container.provide_factory::<dyn PasswordCredentialRepositoryTrait<Id = UniqueId>, _>(|c| {
        let adapter = c.resolve::<UserAccountSqlxPostgresAdapter<L>>()?;
        let encryptor = c.resolve::<dyn Encryptor>()?;
        Ok(Arc::new(PasswordCredentialRepository::new(
            adapter, encryptor,
        )))
    });

    container.provide_factory::<dyn RestorePasswordTokenRepositoryTrait<Id = UniqueId>, _>(|c| {
        let adapter = c.resolve::<UserAccountSqlxPostgresAdapter<L>>()?;
        Ok(Arc::new(RestorePasswordTokenRepository::new(adapter)))
    });

    container.provide_factory::<dyn EmailConfirmationTokenRepositoryTrait<Id = UniqueId>, _>(|c| {
        let adapter = c.resolve::<UserAccountSqlxPostgresAdapter<L>>()?;
        Ok(Arc::new(EmailConfirmationTokenRepository::new(adapter)))
    });

    container.provide_factory::<dyn GuestGeneratorTrait<Id = UniqueId>, _>(|_c| {
        Ok(Arc::new(StringTokenGenerator::new()))
    });

    container.provide_factory::<dyn MemberGeneratorTrait<Id = UniqueId>, _>(move |_c| {
        Ok(Arc::new(JwtTokenGenerator::new(
            &member_jwt_token_secret,
            member_jwt_token_ttl,
        )))
    });

    container.provide_factory::<dyn GuestRegistersTrait<Id = UniqueId>, _>(|c| {
        let uc = c.resolve::<GuestRegistersUc<UniqueId>>()?;
        Ok(Arc::new(GuestRegistersHandler::new(uc)))
    });

    container.provide_factory::<dyn GuestRequestsRestoreTokenTrait<Id = UniqueId>, _>(|c| {
        let uc = c.resolve::<GuestRequestsRestoreTokenUc<UniqueId, L>>()?;
        let logger = c.resolve::<L>()?;
        let transformer = Arc::new(DtoToGuestRequestsRestoreTokenParams::new());
        Ok(Arc::new(GuestRequestsRestoreTokenHandler::new(
            uc,
            logger,
            transformer,
        )))
    });

    container.provide_factory::<dyn GuestWithTokenChangesPasswordTrait<Id = UniqueId>, _>(|c| {
        let uc = c.resolve::<GuestWithTokenChangesPasswordUc<UniqueId, L>>()?;
        let transformer = Arc::new(DtoToGuestWithTokenChangesPasswordParams::new());
        Ok(Arc::new(GuestWithTokenChangesPasswordHandler::new(
            uc,
            transformer,
        )))
    });

    container.provide_factory::<dyn MemberLoginsWithPasswordTrait<Id = UniqueId>, _>(|c| {
        let uc = c.resolve::<MemberLoginsWithPasswordUc<UniqueId, L>>()?;
        let transformer = Arc::new(DtoToMemberLoginsWithPasswordParams::new());
        Ok(Arc::new(MemberLoginsWithPasswordHandler::new(
            uc,
            transformer,
        )))
    });

    container.provide_factory::<dyn MemberRegistersWithPasswordTrait<Id = UniqueId>, _>(|c| {
        let uc = c.resolve::<MemberRegistersWithPasswordUc<UniqueId>>()?;
        let transformer = Arc::new(DtoToMemberRegistersWithPasswordParams::new());
        Ok(Arc::new(MemberRegistersWithPasswordHandler::new(
            uc,
            transformer,
        )))
    });

    container.provide_factory::<dyn MemberVerifiesEmailTrait<Id = UniqueId>, _>(|c| {
        let uc = c.resolve::<dyn MemberConfirmsEmailUcTrait<Id = UniqueId>>()?;
        let logger = c.resolve::<L>()?;
        Ok(Arc::new(MemberVerifiesEmailHandler::new(uc, logger)))
    });
    Ok(())
}
