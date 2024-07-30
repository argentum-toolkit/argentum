use crate::db::adapter::DbAdapterError;
use crate::db::slqx_postgres::migration::collection::MigrationCollection;
use crate::db::slqx_postgres::migration::MigrationDto;
use crate::db::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use sqlx::types::chrono::Utc;
use sqlx::Transaction;
use sqlx_postgres::Postgres;
use std::sync::Arc;

pub struct Migrator<'a> {
    adapter: Arc<SqlxPostgresAdapter>,
    migrations: MigrationCollection<'a>,
    migration_table_name: &'a str,
    logger: Arc<dyn LoggerTrait>,
}

impl<'a> Migrator<'a> {
    pub fn new(
        adapter: Arc<SqlxPostgresAdapter>,
        migrations: MigrationCollection<'a>,
        migration_table_name: &'a str,
        logger: Arc<dyn LoggerTrait>,
    ) -> Self {
        Self {
            adapter,
            migrations,
            migration_table_name,
            logger,
        }
    }

    async fn create_migration_table(&self) -> Result<(), String> {
        self.logger
            .info("Ensuring that migration table exists...".to_string());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (\
                id INT PRIMARY KEY generated always as identity, \
                executed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_DATE, \
                version VARCHAR NOT NULL UNIQUE\
            );",
            self.migration_table_name
        );

        let query = sqlx::query(&sql);
        let res = self.adapter.exec(query).await;
        match res {
            Ok(_) => {
                self.logger.info("Migration table is ensured.".to_string());
                Ok(())
            }
            Err(e) => {
                self.logger
                    .critical(format!("Ensuring was failed with error: {}", e));
                Err(e.to_string())
            }
        }
    }

    async fn rollback(&self, tx: Transaction<'a, Postgres>) -> Result<(), String> {
        if let Err(rollback_error) = self.adapter.rollback(tx).await {
            self.logger
                .critical(format!("Can't rollback transaction: {}", rollback_error));

            return Err(rollback_error.to_string());
        }

        Ok(())
    }

    async fn rollback_with_error(
        &self,
        tx: Transaction<'a, Postgres>,
        e: DbAdapterError,
    ) -> Result<(), String> {
        self.logger.critical(e.to_string());

        self.rollback(tx).await?;

        Err(e.to_string())
    }

    pub async fn migrate_one(&self, version: &str, migration: &Vec<String>) -> Result<(), String> {
        self.logger.info(format!("Migrate version {}", version));

        let tx_res = self.adapter.begin_transaction().await;
        if let Err(e) = tx_res {
            self.logger
                .critical(format!("Can't start transaction: {}", e));

            return Err(e.to_string());
        }

        let mut tx = tx_res.unwrap();

        let sql = format!(
            "SELECT * FROM {} WHERE version = $1 LIMIT 1;",
            self.migration_table_name
        );

        let query = sqlx::query_as(&sql).bind(version);

        let result: Result<Option<MigrationDto>, DbAdapterError> =
            self.adapter.fetch_one(query).await;

        if let Err(e) = result {
            return self.rollback_with_error(tx, e).await;
        }

        if let Ok(Some(_)) = result {
            self.logger
                .info(format!("Migration {} already migrated", version));

            self.rollback(tx).await?;

            return Ok(());
        }

        let dto = MigrationDto {
            version: version.to_string(),
            executed_at: Utc::now(),
        };

        let sql = format!(
            "INSERT INTO {} (version, executed_at) values($1, $2);",
            self.migration_table_name
        );

        let query = sqlx::query(&sql).bind(&dto.version).bind(dto.executed_at);

        let result = self.adapter.exec_with_executor(query, &mut *tx).await;

        if let Err(e) = result {
            return self.rollback_with_error(tx, e).await;
        }

        for sql in migration {
            let query = sqlx::query(sql);
            let result = self.adapter.exec_with_executor(query, &mut *tx).await;

            if let Err(e) = result {
                return self.rollback_with_error(tx, e).await;
            }
        }

        let res = self.adapter.commit(tx).await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                self.logger
                    .critical(format!("Can't commit transaction: {}", e));

                Err(e.to_string())
            }
        }
    }

    pub async fn migrate(&self) -> Result<(), String> {
        self.create_migration_table().await?;

        for (key, migration) in &self.migrations {
            self.migrate_one(key, migration).await?;
        }

        Ok(())
    }
}
