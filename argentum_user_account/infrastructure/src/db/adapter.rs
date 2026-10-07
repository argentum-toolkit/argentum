use std::ops::Deref;

use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;

pub struct UserAccountSqlxPostgresAdapter<L: LoggerTrait>(pub SqlxPostgresAdapter<L>);

impl<L: LoggerTrait> Deref for UserAccountSqlxPostgresAdapter<L> {
    type Target = SqlxPostgresAdapter<L>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
