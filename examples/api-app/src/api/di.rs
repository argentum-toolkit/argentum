use argentum_log_business::{DefaultLogger, Level};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_rest_infrastructure::service::Server;
use argentum_skeleton_infrastructure::di::SkeletonDiBuilder;
use argentum_standard_business::error::DiError;
use std::sync::Arc;

pub async fn di_factory() -> Result<Arc<Server<DefaultLogger<PrettyWriter>>>, DiError> {
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let skeleton_builder = SkeletonDiBuilder::try_new(logger.clone()).await?;

    let skeleton = skeleton_builder.build().await?;

    Ok(skeleton.server.clone())
}
