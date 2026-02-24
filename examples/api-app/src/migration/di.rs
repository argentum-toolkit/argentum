use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_db_infrastructure::slqx_postgres::migration::{MigrationCollection, Migrator};
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_oidc_infrastructure::db::migration as oidc_migration;
use argentum_user_account_infrastructure::db::migration as user_account_migration;
use argentum_user_infrastructure::db::migration as user_migration;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Env {
    pub ag_oidc_database_url: String,
    pub ag_user_database_url: String,
    pub ag_user_account_database_url: String,
}

pub struct Di<'a, L>
where
    L: LoggerTrait,
{
    pub oidc_migrator: Arc<Migrator<'a, L>>,
    pub user_migrator: Arc<Migrator<'a, L>>,
    pub user_application_migrator: Arc<Migrator<'a, L>>,
}

impl<'a, L> Di<'a, L>
where
    L: LoggerTrait,
{
    pub fn new(
        oidc_migrator: Arc<Migrator<'a, L>>,
        user_migrator: Arc<Migrator<'a, L>>,
        user_application_migrator: Arc<Migrator<'a, L>>,
    ) -> Self {
        Self {
            oidc_migrator,
            user_migrator,
            user_application_migrator,
        }
    }
}

async fn create_migrator<'a, L>(
    database_url: &str,
    max_db_connections: u32,
    migrations: MigrationCollection<'a>,
    logger: Arc<L>,
) -> Result<Migrator<'a, L>, String>
where
    L: LoggerTrait,
{
    let pool = PgPoolOptions::new()
        .max_connections(max_db_connections)
        .connect(database_url)
        .await
        .map_err(|e| format!("Can't create PG connection pool. Error: {e}"))?;

    let adapter = Arc::new(SqlxPostgresAdapter::new(pool, logger.clone()));

    Ok(Migrator::new(adapter, migrations, "ag__migrations", logger))
}

pub async fn di_factory<'a>() -> Result<Di<'a, DefaultLogger<PrettyWriter>>, String> {
    let env: Env = envy::from_env::<Env>().map_err(|e| format!("Cant load env. Error: {e}"))?;

    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let max_db_connections = 5;

    let oidc_migrations = oidc_migration::up("ag_oidc_");
    let oidc_migrator = Arc::new(
        create_migrator(
            &env.ag_oidc_database_url,
            max_db_connections,
            oidc_migrations,
            logger.clone(),
        )
        .await?,
    );

    let u_migrations = user_migration::up("ag_user_");
    let u_migrator = Arc::new(
        create_migrator(
            &env.ag_user_database_url,
            max_db_connections,
            u_migrations,
            logger.clone(),
        )
        .await?,
    );

    let ua_migrations = user_account_migration::up("ag_user_account_");
    let ua_migrator = Arc::new(
        create_migrator(
            &env.ag_user_account_database_url,
            max_db_connections,
            ua_migrations,
            logger,
        )
        .await?,
    );

    Ok(Di::new(oidc_migrator, u_migrator, ua_migrator))
}
