use crate::data_type::{Request, Response};
use crate::service::{ErrorHandler, ResponseToJsonTransformer, RouterTrait};
use argentum_log_business::LoggerTrait;

use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::TcpListener;
use tokio_rustls::rustls::ServerConfig;

pub struct Http2Server<L>
where
    L: LoggerTrait,
{
    //config
    addr: SocketAddr,

    cert_file_path: String,
    key_file_path: String,

    //service dependencies
    router: Arc<dyn RouterTrait>,

    response_transformer: Arc<ResponseToJsonTransformer>,

    error_handler: Arc<ErrorHandler<L>>,

    logger: Arc<L>,
}

impl<L> Http2Server<L>
where
    L: LoggerTrait + 'static,
{
    pub fn new(
        addr: SocketAddr,
        cert_file_path: &str,
        key_file_path: &str,
        router: Arc<dyn RouterTrait>,
        response_transformer: Arc<ResponseToJsonTransformer>,
        error_handler: Arc<ErrorHandler<L>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            addr,
            cert_file_path: cert_file_path.into(),
            key_file_path: key_file_path.into(),
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

        //TODO: don't hardcode cert path;
        let cert_file = File::open(&self.cert_file_path)?;
        let key_file = File::open(&self.key_file_path)?;

        let cert_chain: Vec<CertificateDer<'static>> =
            rustls_pemfile::certs(&mut BufReader::new(cert_file)).collect::<Result<_, _>>()?;

        let mut keys: Vec<PrivateKeyDer<'static>> =
            rustls_pemfile::pkcs8_private_keys(&mut BufReader::new(key_file))
                .map(|key| key.map(Into::into))
                .collect::<Result<_, _>>()?;

        if keys.is_empty() {
            return Err("No private keys found".into());
        }

        let mut config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, keys.remove(0))?;

        config.alpn_protocols = vec![b"h2".to_vec()];

        let acceptor = tokio_rustls::TlsAcceptor::from(Arc::new(config));

        loop {
            let (stream, _) = listener.accept().await?;
            let acceptor = acceptor.clone();

            // let io = TokioIo::new(stream);

            let router = self.router.clone();
            let transformer = self.response_transformer.clone();
            let error_handler = self.error_handler.clone();
            let logger = self.logger.clone();

            tokio::task::spawn(async move {
                logger.trace("HTTP/2 request accepted"); //TODO: it is connection. not a request

                let tls_stream = match acceptor.accept(stream).await {
                    Ok(s) => s,
                    Err(err) => {
                        logger.error(format!("TLS handshake failed: {:?}", err));
                        return;
                    }
                };

                let io = TokioIo::new(tls_stream);

                let start = Instant::now();

                let log = logger.clone();
                if let Err(err) = http2::Builder::new(TokioExecutor::new())
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
