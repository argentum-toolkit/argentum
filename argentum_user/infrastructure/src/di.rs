use crate::db::UserSqlxPostgresAdapter;
use crate::db::repository::GuestBindingRepository;
use crate::db::repository::GuestRepository;
use crate::db::repository::MemberRepository;
use crate::db::repository::SessionRepository;
use crate::rest::handler::GetUserHandler;
use crate::service::MemberAuthenticator;
use crate::service::TokenRouter;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_standard_business::{
    data_type::id::Id,
    di_container::{Container, DiError},
};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::repository::{
    guest_binding_repository::GuestBindingRepositoryTrait,
    session_repository::SessionRepositoryTrait,
    user_repository::{GuestUserRepositoryTrait, MemberRepositoryTrait},
};
use argentum_user_business::{
    service::{MemberAuthenticatorTrait, TokenRouterTrait},
    use_case::GetUserUc,
};

use argentum_user_rest::server::handler::GetUserTrait;
use argentum_user_rest::user_rest_provide;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub async fn user_infrastructure_provide<I, L>(
    container: &mut Container,
    connection_url: impl Into<String>,
    max_db_connections: u32,
) -> Result<(), DiError>
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    user_rest_provide::<I>(container, "/api/v1");

    let connection_url = connection_url.into();
    let pool = PgPoolOptions::new()
        .max_connections(max_db_connections)
        .min_connections(2)
        .connect(&connection_url)
        .await
        .map_err(anyhow::Error::from)?;

    container.provide_factory::<UserSqlxPostgresAdapter<L>, _>(move |c| {
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(UserSqlxPostgresAdapter(SqlxPostgresAdapter::new(
            pool.clone(),
            logger,
        ))))
    });

    container.provide_factory::<dyn GuestBindingRepositoryTrait<Id = UniqueId>, _>(|c| {
        let adapter = c.resolve::<UserSqlxPostgresAdapter<L>>()?;

        Ok(Arc::new(GuestBindingRepository::new(adapter)))
    });

    container.provide_factory::<dyn GuestUserRepositoryTrait<Id = UniqueId>, _>(|c| {
        let adapter = c.resolve::<UserSqlxPostgresAdapter<L>>()?;

        Ok(Arc::new(GuestRepository::new(adapter)))
    });

    container.provide_factory::<dyn MemberRepositoryTrait<Id = UniqueId>, _>(|c| {
        let adapter = c.resolve::<UserSqlxPostgresAdapter<L>>()?;

        Ok(Arc::new(MemberRepository::new(adapter)))
    });

    container.provide_factory::<dyn SessionRepositoryTrait<Id = UniqueId>, _>(|c| {
        let adapter = c.resolve::<UserSqlxPostgresAdapter<L>>()?;

        Ok(Arc::new(SessionRepository::new(adapter)))
    });

    container.provide_factory::<dyn TokenRouterTrait, _>(|_c| Ok(Arc::new(TokenRouter::new())));

    container.provide_factory::<dyn MemberAuthenticatorTrait<Id = UniqueId>, _>(|c| {
        let member_repository = c.resolve::<dyn MemberRepositoryTrait<Id = UniqueId>>()?;
        let session_repository = c.resolve::<dyn SessionRepositoryTrait<Id = UniqueId>>()?;

        Ok(Arc::new(MemberAuthenticator::new(
            member_repository,
            session_repository,
        )))
    });

    container.provide_factory::<dyn GetUserTrait<Id = UniqueId>, _>(|c| {
        let get_user_uc = c.resolve::<GetUserUc<UniqueId>>()?;

        Ok(Arc::new(GetUserHandler::new(get_user_uc)))
    });

    Ok(())
}
