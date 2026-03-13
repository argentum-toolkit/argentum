use std::sync::{Arc, Mutex};

use argentum_db_infrastructure::slqx_postgres::{
    SqlxPostgresAdapter,
    migration::{MigrationCollection, Migrator},
};
use argentum_log_business::LoggerTrait;
use argentum_oidc_infrastructure::db::migration as oidc_migration;
use argentum_user_account_infrastructure::db::migration as user_account_migration;
use argentum_user_infrastructure::db::migration as user_migration;

use argentum_standard_business::error::DiError;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;

#[derive(Deserialize, Debug)]
struct Env {
    pub ag_oidc_database_url: String,
    pub ag_user_database_url: String,
    pub ag_user_account_database_url: String,
}

pub struct SkeletonMigrationDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    logger: Arc<L>,
    migrators: Arc<Mutex<Vec<Arc<Migrator<L>>>>>,
}

impl<L> SkeletonMigrationDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    pub fn new(logger: Arc<L>) -> Self {
        Self {
            logger,
            migrators: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn with_migrator(
        &self,
        database_url: &str,
        max_db_connections: u32,
        migrations: MigrationCollection,
    ) -> Result<&Self, DiError>
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

        let adapter = Arc::new(SqlxPostgresAdapter::new(pool, self.logger.clone()));

        self.migrators.lock()?.push(Arc::new(Migrator::new(
            adapter,
            migrations,
            "ag__migrations",
            self.logger.clone(),
        )));

        Ok(self)
    }

    pub async fn with_default_migrators(&self) -> Result<(), DiError>
    where
        L: LoggerTrait,
    {
        let env: Env = envy::from_env::<Env>().map_err(|e| format!("Cant load env. Error: {e}"))?;

        let max_db_connections = 5;

        self.with_migrator(
            &env.ag_user_database_url,
            max_db_connections,
            user_migration::up("ag_user_"),
        )
        .await?;

        self.with_migrator(
            &env.ag_user_account_database_url,
            max_db_connections,
            user_account_migration::up("ag_user_account_"),
        )
        .await?;

        self.with_migrator(
            &env.ag_oidc_database_url,
            max_db_connections,
            oidc_migration::up("ag_oidc_"),
        )
        .await?;

        Ok(())
    }

    pub fn build(self) -> Vec<Arc<Migrator<L>>> {
        let mut migrators: Vec<Arc<Migrator<L>>> = vec![];

        for migrator in self.migrators.clone().lock().unwrap().iter() {
            migrators.push(migrator.clone());
        }

        migrators
    }
}
