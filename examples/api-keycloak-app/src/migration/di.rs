use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_db_infrastructure::slqx_postgres::migration::{MigrationCollection, Migrator};
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_user_account_infrastructure::db::migration as user_account_migration;
use argentum_user_infrastructure::db::migration as user_migration;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Env {
    pub ag_user_database_url: String,
    pub ag_user_account_database_url: String,
}

pub struct Di<L>
where
    L: LoggerTrait,
{
    pub user_migrator: Arc<Migrator<L>>,
    pub user_application_migrator: Arc<Migrator<L>>,
}

impl<L> Di<L>
where
    L: LoggerTrait,
{
    pub fn new(
        user_migrator: Arc<Migrator<L>>,
        user_application_migrator: Arc<Migrator<L>>,
    ) -> Self {
        Self {
            user_migrator,
            user_application_migrator,
        }
    }
}

async fn create_migrator<L>(
    database_url: &str,
    max_db_connections: u32,
    migrations: MigrationCollection,
    logger: Arc<L>,
) -> Result<Migrator<L>, String>
where
    L: LoggerTrait,
{
    let pool = PgPoolOptions::new()
        .max_connections(max_db_connections)
        .connect(database_url)
        .await
        .map_err(|e| {
            format!(
                "Can't create PG connection pool for connection `{}`. Error: {e}",
                database_url
            )
        })?;

    let adapter = Arc::new(SqlxPostgresAdapter::new(pool, logger.clone()));

    Ok(Migrator::new(adapter, migrations, "ag__migrations", logger))
}

pub async fn di_factory() -> Result<Di<DefaultLogger<PrettyWriter>>, String> {
    let env: Env = envy::from_env::<Env>().map_err(|e| format!("Cant load env. Error: {e}"))?;

    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let max_db_connections = 5;

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

    Ok(Di::new(u_migrator, ua_migrator))
}
