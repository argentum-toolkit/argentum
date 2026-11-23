use std::sync::Arc;

use argentum_contact_us_infrastructure::db::migration as contact_us_migration;
use argentum_db_infrastructure::slqx_postgres::{
    SqlxPostgresAdapter,
    migration::{MigrationCollection, Migrator},
};
use argentum_log_business::LoggerTrait;
use argentum_log_infrastructure::log_bridge::LogBridge;
use argentum_oidc_infrastructure::db::migration as oidc_migration;
use argentum_user_account_infrastructure::db::migration as user_account_migration;
use argentum_user_infrastructure::db::migration as user_migration;

use argentum_standard_business::di_container::DiError;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;

#[derive(Deserialize, Debug)]
struct Env {
    pub ag_oidc_database_url: String,
    pub ag_user_database_url: String,
    pub ag_user_account_database_url: String,
    pub ag_contact_us_database_url: String,
}

async fn create_migrator<L>(
    logger: Arc<L>,
    database_url: &str,
    max_db_connections: u32,
    migrations: MigrationCollection,
    component_name: &str,
) -> Result<Arc<Migrator<L>>, DiError>
where
    L: LoggerTrait + 'static,
{
    let pool = PgPoolOptions::new()
        .max_connections(max_db_connections)
        .connect(database_url)
        .await
        .map_err(|e| {
            DiError::with_context(
                format!("Can't connect to the `{component_name}` database"),
                e,
            )
        })?;

    let adapter = Arc::new(SqlxPostgresAdapter::new(pool, logger.clone()));

    Ok(Arc::new(Migrator::new(
        adapter,
        migrations,
        "ag__migrations",
        logger,
    )))
}

pub async fn skeleton_migration_provide<L>(logger: Arc<L>) -> Result<Vec<Arc<Migrator<L>>>, DiError>
where
    L: LoggerTrait + 'static,
{
    LogBridge::install(logger.clone());

    let env: Env =
        envy::from_env::<Env>().map_err(|e| DiError::with_context("Can't load environment", e))?;
    let max_db_connections = 5;

    let user_migrator = create_migrator(
        logger.clone(),
        &env.ag_user_database_url,
        max_db_connections,
        user_migration::up("ag_user_"),
        "user",
    )
    .await?;

    let user_account_migrator = create_migrator(
        logger.clone(),
        &env.ag_user_account_database_url,
        max_db_connections,
        user_account_migration::up("ag_user_account_"),
        "user_account",
    )
    .await?;

    let oidc_migrator = create_migrator(
        logger.clone(),
        &env.ag_oidc_database_url,
        max_db_connections,
        oidc_migration::up("ag_oidc_"),
        "oidc",
    )
    .await?;

    let contact_us_migrator = create_migrator(
        logger,
        &env.ag_contact_us_database_url,
        max_db_connections,
        contact_us_migration::up("ag_contact_us_"),
        "contact_us",
    )
    .await?;

    Ok(vec![
        user_migrator,
        user_account_migrator,
        oidc_migrator,
        contact_us_migrator,
    ])
}
