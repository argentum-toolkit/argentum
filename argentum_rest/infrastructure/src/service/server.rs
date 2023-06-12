use crate::data_type::{Request, Response};
use crate::service::{ErrorHandler, ResponseToJsonTransformer, Router};
use argentum_log_business::LoggerTrait;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::TcpListener;

pub struct Server {
    //config
    addr: SocketAddr,

    //service dependencies
    router: Arc<dyn Router>,

    response_transformer: Arc<ResponseToJsonTransformer>,

    error_handler: Arc<ErrorHandler>,

    logger: Arc<dyn LoggerTrait>,
}

impl Server {
    pub fn new(
        addr: SocketAddr,
        router: Arc<dyn Router>,
        response_transformer: Arc<ResponseToJsonTransformer>,
        error_handler: Arc<ErrorHandler>,
        logger: Arc<dyn LoggerTrait>,
    ) -> Self {
        Server {
            addr,
            router,
            response_transformer,
            error_handler,
            logger,
        }
    }

    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        async fn handle(
            req: Request,
            router: Arc<dyn Router>,
            transformer: Arc<ResponseToJsonTransformer>,
            error_handler: Arc<ErrorHandler>,
        ) -> Result<Response, hyper::Error> {
            let res = router.route(req).await;

            let response = match res {
                Ok(r) => r,
                Err(e) => error_handler.handle(e),
            };

            Ok(transformer.transform(response))
        }

        let listener = TcpListener::bind(self.addr).await?;
        println!("Listening on http://{}", self.addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let router = self.router.clone();
            let transformer = self.response_transformer.clone();
            let error_handler = self.error_handler.clone();
            let logger = self.logger.clone();

            tokio::task::spawn(async move {
                let start = Instant::now();
                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        stream,
                        service_fn(move |request| {
                            handle(
                                request,
                                router.clone(),
                                transformer.clone(),
                                error_handler.clone(),
                            )
                        }),
                    )
                    .await
                {
                    println!("Failed to serve connection: {:?}", err);
                }

                let elapsed = start.elapsed();
                logger.trace(format!("Duration: {}μs", elapsed.as_micros()));
            });
        }
    }
}
