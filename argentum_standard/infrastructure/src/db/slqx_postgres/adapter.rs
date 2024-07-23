use crate::db::adapter::DbAdapterError;
use sqlx::postgres::PgPool;
use std::collections::BTreeMap;
use std::future::Future;

use argentum_log_business::LoggerTrait;
use sqlx::query::{Query, QueryAs};
use sqlx::{Error, Execute, Executor, FromRow, Postgres, Transaction};
use sqlx_postgres::{PgArguments, PgRow};
use std::sync::Arc;

pub struct DbRow<'r> {
    pub data: BTreeMap<&'r str, &'r [u8]>,
}

pub trait FromDbRow: Sized {
    fn from_db_row(row: DbRow) -> Result<Self, Error>;
}

pub struct SqlxPostgresAdapter {
    pub pool: Arc<PgPool>,
    logger: Arc<dyn LoggerTrait>,
}

impl SqlxPostgresAdapter {
    pub fn new(pool: Arc<PgPool>, logger: Arc<dyn LoggerTrait>) -> Self {
        Self { pool, logger }
    }

    pub fn exec<'q>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> impl Future<Output = Result<u64, DbAdapterError>> + Send + 'q {
        self.exec_with_executor(query, &*self.pool)
    }

    /// Execute query with Executor (transaction or connection pool)
    ///
    /// # Examples
    /// use argentum_standard_infrastructure::db::slqx_postgres::SqlxPostgresAdapter;
    /// use futures::executor::block_on;
    /// let adapter: SqlxPostgresAdapter = todo!();
    /// let mut tx = block_on(adapter.begin_transaction()).unwrap();
    /// let query = sqlx::query("some query");
    /// let result = block_on(adapter.exec_with_executor(query, &mut *tx));
    /// let t_result = block_on(adapter.commit(tx));
    /// ```
    pub fn exec_with_executor<'q, E>(
        &'q self,
        query: Query<'q, Postgres, PgArguments>,
        executor: E,
    ) -> impl Future<Output = Result<u64, DbAdapterError>> + Send + 'q
    where
        E: Executor<'q, Database = Postgres> + 'q,
    {
        async move {
            let sql = query.sql().to_string();
            self.logger.debug(sql.clone());
            let result = query.execute(executor).await;
            self.logger.trace("done".to_string());
            match result {
                Ok(r) => Ok(r.rows_affected()),
                Err(e) => Err(DbAdapterError {
                    msg: e.to_string(),
                    sql: Some(sql),
                }),
            }
        }
    }

    //TODO: fetch with executor
    pub fn fetch_one<'q, F>(
        &'q self,
        query_as: QueryAs<'q, Postgres, F, PgArguments>,
    ) -> impl Future<Output = Result<Option<F>, DbAdapterError>> + Send + 'q
    where
        F: Send + Unpin + for<'r> FromRow<'r, PgRow> + 'q,
    {
        async move {
            let sql = query_as.sql().to_string();
            self.logger.debug(sql.clone());
            let result: Result<Option<F>, Error> = query_as.fetch_optional(&*self.pool).await;
            self.logger.trace("done".to_string());

            match result {
                Ok(Some(r)) => Ok(Some(r)),
                Ok(None) => Ok(None),
                Err(e) => Err(DbAdapterError {
                    msg: e.to_string(),
                    sql: Some(sql),
                }),
            }
        }
    }

    pub fn begin_transaction(
        &self,
    ) -> impl Future<Output = Result<Transaction<'static, Postgres>, DbAdapterError>> + Send + '_
    {
        async move {
            self.logger.debug("Begin transaction".to_string());
            match self.pool.begin().await {
                Ok(tx) => Ok(tx),
                Err(e) => Err(DbAdapterError {
                    msg: e.to_string(),
                    sql: None,
                }),
            }
        }
    }

    pub fn commit<'a>(
        &'a self,
        tx: Transaction<'a, Postgres>,
    ) -> impl Future<Output = Result<(), DbAdapterError>> + Send + 'a {
        async move {
            self.logger.debug("Commit transaction".to_string());
            match tx.commit().await {
                Ok(_) => Ok(()),
                Err(e) => Err(DbAdapterError {
                    msg: e.to_string(),
                    sql: None,
                }),
            }
        }
    }

    pub fn rollback<'a>(
        &'a self,
        tx: Transaction<'a, Postgres>,
    ) -> impl Future<Output = Result<(), DbAdapterError>> + Send + 'a {
        async move {
            self.logger.debug("Rollback transaction".to_string());
            match tx.rollback().await {
                Ok(_) => Ok(()),
                Err(e) => Err(DbAdapterError {
                    msg: e.to_string(),
                    sql: None,
                }),
            }
        }
    }
}
