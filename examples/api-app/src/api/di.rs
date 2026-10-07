use argentum_log_business::{DefaultLogger, Level};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_notification_business::{NotifierTrait, mock::StdoutNotifier};

use argentum_rest_infrastructure::service::server::Http1Server;
use argentum_skeleton_infrastructure::di::skeleton_api_provide;
use argentum_standard_business::{
    data_type::id::IdFactory, di_container::Container, error::DiError,
};
use argentum_standard_infrastructure::data_type::unique_id::{UniqueId, UniqueIdFactory};

use std::sync::Arc;

pub async fn di_factory() -> Result<Arc<Http1Server<DefaultLogger<PrettyWriter>>>, DiError> {
    // let log_writer = Arc::new(PrettyWriter::new());
    // let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let mut container = Container::new();

    // user_infrastructure_provide(&mut container);

    register_logger(&mut container);
    register_id_factory(&mut container);
    register_notifier(&mut container);

    // let listen = "172.18.0.1:8088";
    // let listen = "127.0.0.1:8088";
    skeleton_api_provide::<DefaultLogger<PrettyWriter>>(&mut container, "0.0.0.0:8088").await?;

    let http1_server: Arc<_> = container
        .resolve::<Http1Server<DefaultLogger<PrettyWriter>>>()
        .unwrap();

    Ok(http1_server)

    // let skeleton_builder = SkeletonApiDiBuilder::try_new("0.0.0.0:8088", logger.clone())?;

    // let skeleton = skeleton_builder.build(vec![])?;

    // Ok(skeleton.http1_server.clone())
}

fn register_logger(container: &mut Container) {
    container.provide_factory::<DefaultLogger<PrettyWriter>, _>(|_c| {
        let log_writer = Arc::new(PrettyWriter::new());

        Ok(Arc::new(DefaultLogger::new(Level::Trace, log_writer)))
    });
}

fn register_id_factory(container: &mut Container) {
    container.provide_factory::<dyn IdFactory<Id = UniqueId>, _>(|_c| {
        Ok(Arc::new(UniqueIdFactory::new()))
    });
}

fn register_notifier(container: &mut Container) {
    container.provide_factory::<dyn NotifierTrait<Id = UniqueId>, _>(|_c| {
        Ok(Arc::new(StdoutNotifier::new()))
    });
}

// pub fn di_factory_http2() -> Result<Arc<Http2Server<DefaultLogger<PrettyWriter>>>, DiError> {
//     let log_writer = Arc::new(PrettyWriter::new());
//     let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

//     let skeleton_builder = SkeletonApiDiBuilder::try_new("0.0.0.0:8088", logger.clone()).await?;

//     let skeleton = skeleton_builder.build(vec![]).await?;

//     Ok(skeleton.http2_server.clone())
// }
