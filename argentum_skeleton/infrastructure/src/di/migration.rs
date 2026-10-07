use std::sync::Arc;

use argentum_db_infrastructure::slqx_postgres::{
    SqlxPostgresAdapter,
    migration::{MigrationCollection, Migrator},
};
use argentum_log_business::LoggerTrait;
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
}

async fn create_migrator<L>(
    logger: Arc<L>,
    database_url: &str,
    max_db_connections: u32,
    migrations: MigrationCollection,
) -> Result<Arc<Migrator<L>>, DiError>
where
    L: LoggerTrait + 'static,
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
    let env: Env = envy::from_env::<Env>().map_err(|e| format!("Cant load env. Error: {e}"))?;
    let max_db_connections = 5;

    let user_migrator = create_migrator(
        logger.clone(),
        &env.ag_user_database_url,
        max_db_connections,
        user_migration::up("ag_user_"),
    )
    .await?;

    let user_account_migrator = create_migrator(
        logger.clone(),
        &env.ag_user_account_database_url,
        max_db_connections,
        user_account_migration::up("ag_user_account_"),
    )
    .await?;

    let oidc_migrator = create_migrator(
        logger,
        &env.ag_oidc_database_url,
        max_db_connections,
        oidc_migration::up("ag_oidc_"),
    )
    .await?;

    Ok(vec![user_migrator, user_account_migrator, oidc_migrator])
}
