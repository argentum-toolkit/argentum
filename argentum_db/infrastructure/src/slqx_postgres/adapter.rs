use crate::adapter::DbAdapterError;
use sqlx::postgres::PgPool;
use std::collections::BTreeMap;
use std::future::Future;

use argentum_log_business::LoggerTrait;
use sqlx::query::{Query, QueryAs};
use sqlx::{Error, Executor, FromRow, Postgres, Transaction};
use sqlx_postgres::{PgArguments, PgRow};
use std::sync::Arc;

pub struct DbRow<'r> {
    pub data: BTreeMap<&'r str, &'r [u8]>,
}

pub trait FromDbRow: Sized {
    fn from_db_row(row: DbRow) -> Result<Self, Error>;
}

pub struct SqlxPostgresAdapter<L>
where
    L: LoggerTrait,
{
    pub pool: PgPool,
    logger: Arc<L>,
}

impl<L> SqlxPostgresAdapter<L>
where
    L: LoggerTrait,
{
    pub fn new(pool: PgPool, logger: Arc<L>) -> Self {
        Self { pool, logger }
    }

    pub fn exec<'q>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> impl Future<Output = Result<u64, DbAdapterError>> + Send + 'q {
        self.exec_with_executor(query, &self.pool)
    }

    /// Execute query with Executor (transaction or connection pool)
    ///
    /// # Examples
    /// ```rust,ignore
    /// use argentum_db_infrastructure::sqlx_postgres::SqlxPostgresAdapter;
    /// let adapter: SqlxPostgresAdapter = todo!();
    /// let mut tx = adapter.begin_transaction().await.unwrap();
    /// let query = sqlx::query("some query");
    /// let result = adapter.exec_with_executor(query, &mut *tx).await;
    /// let t_result = adapter.commit(tx).await;
    /// ```
    pub async fn exec_with_executor<'q, E>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
        executor: E,
    ) -> Result<u64, DbAdapterError>
    where
        E: Executor<'q, Database = Postgres> + 'q,
    {
        let result = query.execute(executor).await;
        match result {
            Ok(r) => Ok(r.rows_affected()),
            Err(e) => Err(DbAdapterError {
                msg: e.to_string(),
                sql: None,
            }),
        }
    }

    //TODO: fetch with executor
    pub async fn fetch_one<'q, F>(
        &'q self,
        query_as: QueryAs<'q, Postgres, F, PgArguments>,
    ) -> Result<Option<F>, DbAdapterError>
    where
        F: Send + Unpin + for<'r> FromRow<'r, PgRow> + 'q,
    {
        let result: Result<Option<F>, Error> = query_as.fetch_optional(&self.pool).await;

        match result {
            Ok(Some(r)) => Ok(Some(r)),
            Ok(None) => Ok(None),
            Err(e) => Err(DbAdapterError {
                msg: e.to_string(),
                sql: None,
            }),
        }
    }

    pub async fn fetch_all<'q, F>(
        &'q self,
        query_as: QueryAs<'q, Postgres, F, PgArguments>,
    ) -> Result<Vec<F>, DbAdapterError>
    where
        F: Send + Unpin + for<'r> FromRow<'r, PgRow> + 'q,
    {
        let result: Result<Vec<F>, Error> = query_as.fetch_all(&self.pool).await;

        match result {
            Ok(rows) => Ok(rows),
            Err(e) => Err(DbAdapterError {
                msg: e.to_string(),
                sql: None,
            }),
        }
    }

    pub async fn begin_transaction(
        &self,
    ) -> Result<Transaction<'static, Postgres>, DbAdapterError> {
        self.logger.debug("Begin transaction");

        match self.pool.begin().await {
            Ok(tx) => Ok(tx),
            Err(e) => Err(DbAdapterError {
                msg: e.to_string(),
                sql: None,
            }),
        }
    }

    pub async fn commit<'a>(&'a self, tx: Transaction<'a, Postgres>) -> Result<(), DbAdapterError> {
        self.logger.debug("Commit transaction");

        match tx.commit().await {
            Ok(_) => Ok(()),
            Err(e) => Err(DbAdapterError {
                msg: e.to_string(),
                sql: None,
            }),
        }
    }

    pub async fn rollback<'a>(
        &'a self,
        tx: Transaction<'a, Postgres>,
    ) -> Result<(), DbAdapterError> {
        self.logger.debug("Rollback transaction");

        match tx.rollback().await {
            Ok(_) => Ok(()),
            Err(e) => Err(DbAdapterError {
                msg: e.to_string(),
                sql: None,
            }),
        }
    }
}
