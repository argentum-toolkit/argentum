use argentum_log_business::{DefaultLogger, Level};
use argentum_log_infrastructure::stdout::PrettyWriter;

use argentum_rest_infrastructure::service::server::Http1Server;
// use argentum_rest_infrastructure::service::server::Http2Server;
use argentum_skeleton_infrastructure::di::skeleton_api_provide;
use argentum_standard_business::di_container::{Container, DiError};

use std::sync::Arc;

pub async fn di_factory_http1() -> Result<Arc<Http1Server<DefaultLogger<PrettyWriter>>>, DiError> {
    let mut container = Container::new();
    container.provide_factory::<DefaultLogger<PrettyWriter>, _>(|_c| {
        let log_writer = Arc::new(PrettyWriter::new());

        Ok(Arc::new(DefaultLogger::new(Level::Trace, log_writer)))
    });

    // let listen = "172.18.0.1:8088";
    // let listen = "127.0.0.1:8088";
    skeleton_api_provide::<DefaultLogger<PrettyWriter>>(&mut container, "0.0.0.0:8088").await?;

    let http1_server: Arc<_> = container
        .resolve::<Http1Server<DefaultLogger<PrettyWriter>>>()
        .unwrap();

    Ok(http1_server)
}

// pub async fn di_factory_http2() -> Result<Arc<Http2Server<DefaultLogger<PrettyWriter>>>, DiError> {
//     let mut container = Container::new();
//     container.provide_factory::<DefaultLogger<PrettyWriter>, _>(|_c| {
//         let log_writer = Arc::new(PrettyWriter::new());
//
//         Ok(Arc::new(DefaultLogger::new(Level::Trace, log_writer)))
//     });
//
//     skeleton_api_provide::<DefaultLogger<PrettyWriter>>(&mut container, "0.0.0.0:8088").await?;
//
//     let http2_server: Arc<_> = container
//         .resolve::<Http2Server<DefaultLogger<PrettyWriter>>>()
//         .unwrap();
//
//     Ok(http2_server)
// }
