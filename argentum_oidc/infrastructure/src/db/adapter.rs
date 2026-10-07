use std::ops::Deref;

use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;

pub struct OidcSqlxPostgresAdapter<L: LoggerTrait>(pub SqlxPostgresAdapter<L>);

impl<L: LoggerTrait> Deref for OidcSqlxPostgresAdapter<L> {
    type Target = SqlxPostgresAdapter<L>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
