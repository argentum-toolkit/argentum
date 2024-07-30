use std::env;

use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_standard_infrastructure::db::slqx_postgres::migration::{
    MigrationCollection, Migrator,
};
use argentum_standard_infrastructure::db::slqx_postgres::SqlxPostgresAdapter;
use argentum_user_account_infrastructure::db::migration as user_account_migration;
use argentum_user_infrastructure::db::migration as user_migration;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct DiC<'a> {
    pub user_migrator: Arc<Migrator<'a>>,
    pub user_application_migrator: Arc<Migrator<'a>>,
}

impl<'a> DiC<'a> {
    pub fn new(
        user_migrator: Arc<Migrator<'a>>,
        user_application_migrator: Arc<Migrator<'a>>,
    ) -> DiC<'a> {
        DiC {
            user_migrator,
            user_application_migrator,
        }
    }
}

async fn create_migrator<'a>(
    u_database_url: &str,
    max_db_connections: u32,
    migrations: MigrationCollection<'a>,
    logger: Arc<dyn LoggerTrait>,
) -> Migrator<'a> {
    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(max_db_connections)
            .connect(u_database_url)
            .await
            .unwrap(),
    );

    let adapter = Arc::new(SqlxPostgresAdapter::new(pool, logger.clone()));

    Migrator::new(adapter, migrations, "ag__migrations", logger)
}

pub async fn di_factory<'a>() -> DiC<'a> {
    dotenv().ok();

    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let max_db_connections = 5;
    const U_CONNECTION_URL_ENV_NAME: &str = "AG_USER_DATABASE_URL";

    let u_database_url = env::var(U_CONNECTION_URL_ENV_NAME)
        .unwrap_or_else(|_| panic!("{} must be set", U_CONNECTION_URL_ENV_NAME));

    const UA_CONNECTION_URL_ENV_NAME: &str = "AG_USER_ACCOUNT_DATABASE_URL";
    let ua_database_url = env::var(UA_CONNECTION_URL_ENV_NAME)
        .unwrap_or_else(|_| panic!("{} must be set", UA_CONNECTION_URL_ENV_NAME));

    let u_migrations = user_migration::up("ag_user_");
    let u_migrator = Arc::new(
        create_migrator(
            &u_database_url,
            max_db_connections,
            u_migrations,
            logger.clone(),
        )
        .await,
    );

    let ua_migrations = user_account_migration::up("ag_user_account_");
    let ua_migrator = Arc::new(
        create_migrator(&ua_database_url, max_db_connections, ua_migrations, logger).await,
    );

    DiC::new(u_migrator, ua_migrator)
}
