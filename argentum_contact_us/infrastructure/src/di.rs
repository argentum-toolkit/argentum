use crate::db::ContactUsSqlxPostgresAdapter;
use crate::db::repository::MessageRepository;
use crate::notifier::{EmailAdminNotifier, build_smtp_transport};
use crate::rest::handler::{GuestSendsMessageHandler, MemberSendsMessageHandler};
use crate::rest::transformer::{DtoToGuestSendsMessageParams, DtoToMemberSendsMessageParams};
use argentum_contact_us_business::repository::message_repository::MessageRepositoryTrait;

use argentum_contact_us_business::notifier::admin_notifier::AdminNotifierTrait;
use argentum_contact_us_rest::{
    contact_us_rest_provide,
    server::handler::{GuestSendsMessageTrait, MemberSendsMessageTrait},
};
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::di_container::{Container, DiError};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::repository::user_repository::MemberRepositoryTrait;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[allow(clippy::too_many_arguments)]
pub async fn contact_us_infrastructure_provider<I, L>(
    container: &mut Container,
    connection_url: impl Into<String>,
    max_db_connections: u32,
    admin_email: impl Into<String>,
    email_from: impl Into<String>,
    smtp_host: impl Into<String>,
    smtp_port: u16,
    smtp_tls_mode: impl Into<String>,
    smtp_username: impl Into<String>,
    smtp_password: impl Into<String>,
) -> Result<(), DiError>
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    contact_us_rest_provide::<I>(container, "/api/v1");

    let connection_url = connection_url.into();
    let pool = PgPoolOptions::new()
        .max_connections(max_db_connections)
        .min_connections(2)
        .connect(&connection_url)
        .await
        .map_err(|e| {
            DiError::with_context(
                format!("Can't connect to the `contact_us` PostgreSQL database `{connection_url}`"),
                e,
            )
        })?;

    container.provide_factory::<ContactUsSqlxPostgresAdapter<L>, _>(move |c| {
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(ContactUsSqlxPostgresAdapter(
            SqlxPostgresAdapter::new(pool.clone(), logger),
        )))
    });

    container.provide_factory::<dyn MessageRepositoryTrait<Id = UniqueId>, _>(|c| {
        let adapter = c.resolve::<ContactUsSqlxPostgresAdapter<L>>()?;
        Ok(Arc::new(MessageRepository::new(adapter)))
    });

    let admin_email = admin_email.into();
    let smtp_host = smtp_host.into();

    if admin_email.is_empty() || smtp_host.is_empty() {
        return Err(DiError::from(
            "AG_CONTACT_US_ADMIN_EMAIL and AG_SMTP_HOST are required to notify the admin about new messages",
        ));
    }

    let email_from = email_from.into();
    let smtp_username = smtp_username.into();
    let smtp_password = smtp_password.into();
    let smtp_tls_mode = smtp_tls_mode.into();

    let transport = Arc::new(build_smtp_transport(
        &smtp_host,
        smtp_port,
        &smtp_tls_mode,
        &smtp_username,
        &smtp_password,
    )?);

    container.provide_factory::<dyn AdminNotifierTrait<Id = UniqueId>, _>(move |c| {
        let member_repository = c.resolve::<dyn MemberRepositoryTrait<Id = UniqueId>>()?;

        Ok(Arc::new(EmailAdminNotifier::new(
            email_from.clone(),
            admin_email.clone(),
            member_repository,
            transport.clone(),
        )))
    });

    container.provide_factory::<dyn GuestSendsMessageTrait<Id = UniqueId>, _>(|c| {
        let uc = c.resolve::<argentum_contact_us_business::use_case::guest_sends_message::GuestSendsMessageUc<UniqueId, L>>()?;
        let transformer = Arc::new(DtoToGuestSendsMessageParams::new());
        Ok(Arc::new(GuestSendsMessageHandler::new(
            uc, transformer,
        )))
    });

    container.provide_factory::<dyn MemberSendsMessageTrait<Id = UniqueId>, _>(|c| {
        let uc = c.resolve::<argentum_contact_us_business::use_case::member_sends_message::MemberSendsMessageUc<UniqueId, L>>()?;
        let transformer = Arc::new(DtoToMemberSendsMessageParams::new());
        Ok(Arc::new(MemberSendsMessageHandler::new(
            uc, transformer,
        )))
    });

    Ok(())
}
