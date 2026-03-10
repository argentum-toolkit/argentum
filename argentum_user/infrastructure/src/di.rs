use crate::db::repository::GuestBindingRepository;
use crate::db::repository::GuestRepository;
use crate::db::repository::MemberRepository;
use crate::db::repository::SessionRepository;
use crate::rest::handler::GetUserHandler;
use crate::service::MemberAuthenticator;
use crate::service::TokenRouter;
use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::di::RestDiBuilder;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::error::DiError;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::di::{UserBusinessDi, UserBusinessDiBuilder};
use argentum_user_business::repository::user_repository::MemberRepositoryTrait;
use argentum_user_business::service::MemberAuthenticatorTrait;
use argentum_user_rest::ApiDi;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::sync::Mutex;

pub struct UserInfrastructureDi<I, L>
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    pub business_di: Arc<UserBusinessDi<I, L>>,
    pub api_di: ApiDi<I>,
}

pub struct UserInfrastructureDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    connection_url: String,
    max_db_connections: u32,
    business_builder: Arc<Mutex<UserBusinessDiBuilder<UniqueId, L>>>,
    rest_di: Arc<RestDiBuilder<UniqueId, L>>,
    logger: Arc<L>,

    built_di: Option<Arc<UserInfrastructureDi<UniqueId, L>>>,
}

impl<L> UserInfrastructureDiBuilder<L>
where
    L: LoggerTrait,
{
    pub fn new(
        connection_url: &str,
        max_db_connections: u32,

        business_builder: Arc<Mutex<UserBusinessDiBuilder<UniqueId, L>>>,
        rest_builder: Arc<RestDiBuilder<UniqueId, L>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            connection_url: connection_url.into(),
            max_db_connections,
            business_builder,
            rest_di: rest_builder,
            logger,
            built_di: None,
        }
    }

    pub async fn with_default_services(self) -> Result<Self, DiError> {
        let pool = PgPoolOptions::new()
            .max_connections(self.max_db_connections)
            .min_connections(2)
            .connect(&self.connection_url)
            .await
            .map_err(|e| {
                format!(
                    "Can't create PG connection pool for connection `{}`. Error: {e}",
                    &self.connection_url
                )
            })?;

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool, self.logger.clone()));

        let session_repository = Arc::new(SessionRepository::new(pg_adapter.clone()));
        self.business_builder
            .lock()?
            .with_guest_binding_repository(Arc::new(GuestBindingRepository::new(
                pg_adapter.clone(),
            )))
            .with_guest_repository(Arc::new(GuestRepository::new(pg_adapter.clone())))
            .with_session_repository(session_repository.clone())
            .with_token_router(Arc::new(TokenRouter::new()));

        let member_repo = Arc::new(MemberRepository::new(pg_adapter.clone()));

        self.business_builder
            .lock()?
            .with_member_repository(member_repo.clone());

        self.business_builder
            .lock()?
            .with_member_authenticator(Arc::new(MemberAuthenticator::new(
                member_repo.clone(),
                session_repository,
            )));

        Ok(self)
    }

    pub async fn with_member_repository(
        &mut self,
        member_repository: Arc<dyn MemberRepositoryTrait<Id = UniqueId>>,
    ) -> Result<&mut Self, DiError> {
        self.business_builder
            .lock()?
            .with_member_repository(member_repository.clone());

        Ok(self)
    }

    pub async fn with_member_authenticator(
        &mut self,
        member_authenticator: Arc<dyn MemberAuthenticatorTrait<Id = UniqueId>>,
    ) -> Result<&mut Self, DiError> {
        self.business_builder
            .lock()?
            .with_member_authenticator(member_authenticator);

        Ok(self)
    }

    pub fn build(&mut self) -> Result<Arc<UserInfrastructureDi<UniqueId, L>>, DiError> {
        if let Some(di) = &self.built_di {
            return Ok(di.clone());
        }

        let business_di = self.business_builder.lock()?.build()?;

        let get_user_handler = Arc::new(GetUserHandler::new(business_di.get_user_uc.clone()));

        let rest_di = self.rest_di.build()?;
        let api_di = ApiDi::new(
            "/api/v1",
            rest_di.request_transformer.clone(),
            rest_di.bearer_authenticator.clone(),
            get_user_handler.clone(),
            rest_di.error_pre_handler.clone(),
        );

        let di = Arc::new(UserInfrastructureDi {
            business_di,
            api_di,
        });

        self.built_di = Some(di.clone());

        Ok(di)
    }
}
