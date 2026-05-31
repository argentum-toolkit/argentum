use argentum_log_business::{DefaultLogger, Level};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_rest_infrastructure::service::server::Http2Server;
use argentum_skeleton_infrastructure::di::SkeletonApiDiBuilder;
use argentum_standard_business::error::DiError;
use std::sync::Arc;

pub async fn di_factory() -> Result<Arc<Http2Server<DefaultLogger<PrettyWriter>>>, DiError> {
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    // let listen = "172.18.0.1:8088";
    // let listen = "127.0.0.1:8088";
    let skeleton_builder = SkeletonApiDiBuilder::try_new("0.0.0.0:8088", logger.clone()).await?;

    let skeleton = skeleton_builder.build(vec![]).await?;

    Ok(skeleton.http2_server.clone())
}
