use argentum_db_infrastructure::slqx_postgres::migration::Migrator;
use argentum_log_business::{DefaultLogger, Level};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_skeleton_infrastructure::di::SkeletonMigrationDiBuilder;
use argentum_standard_business::error::DiError;
use std::sync::Arc;

pub async fn di_factory() -> Result<Vec<Arc<Migrator<DefaultLogger<PrettyWriter>>>>, DiError> {
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let builder = SkeletonMigrationDiBuilder::new(logger.clone());
    builder.with_default_migrators().await?;

    Ok(builder.build())
}
