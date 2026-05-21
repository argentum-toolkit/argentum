use crate::data_type::{Request, Response};
use crate::service::{ErrorHandler, ResponseToJsonTransformer, RouterTrait};
use argentum_log_business::LoggerTrait;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::TcpListener;

pub struct Server<L>
where
    L: LoggerTrait,
{
    //config
    addr: SocketAddr,

    //service dependencies
    router: Arc<dyn RouterTrait>,

    response_transformer: Arc<ResponseToJsonTransformer>,

    error_handler: Arc<ErrorHandler<L>>,

    logger: Arc<L>,
}

impl<L> Server<L>
where
    L: LoggerTrait + 'static,
{
    pub fn new(
        addr: SocketAddr,
        router: Arc<dyn RouterTrait>,
        response_transformer: Arc<ResponseToJsonTransformer>,
        error_handler: Arc<ErrorHandler<L>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            addr,
            router,
            response_transformer,
            error_handler,
            logger,
        }
    }

    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        async fn handle<T>(
            req: Request,
            router: Arc<dyn RouterTrait>,
            transformer: Arc<ResponseToJsonTransformer>,
            error_handler: Arc<ErrorHandler<T>>,
            logger: Arc<T>,
        ) -> Result<Response, String>
        where
            T: LoggerTrait,
        {
            let res = router.route(req).await;

            let response = match res {
                Ok(r) => r,
                Err(e) => error_handler.handle(e),
            };

            match transformer.transform(response) {
                Ok(r) => Ok(r),
                Err(e) => {
                    logger.critical(e.clone());

                    Err(e)
                }
            }
        }

        let listener = TcpListener::bind(self.addr).await?;
        println!("Listening on http://{}", self.addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            let router = self.router.clone();
            let transformer = self.response_transformer.clone();
            let error_handler = self.error_handler.clone();
            let logger = self.logger.clone();

            tokio::task::spawn(async move {
                logger.trace("HTTP request accepted"); //TODO: it is connection. not a request
                let start = Instant::now();

                let log = logger.clone();
                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        io,
                        service_fn(move |request| {
                            handle(
                                request,
                                router.clone(),
                                transformer.clone(),
                                error_handler.clone(),
                                log.clone(),
                            )
                        }),
                    )
                    .await
                {
                    println!("Failed to serve connection: {err:?}");
                }

                let elapsed = start.elapsed();
                logger.trace(format!("Duration: {}μs", elapsed.as_micros()));
            });
        }
    }
}
