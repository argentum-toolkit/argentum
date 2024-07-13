use crate::data_type::{Request, Response};
use crate::service::{ErrorHandler, ResponseToJsonTransformer, RouterTrait};
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
    router: Arc<dyn RouterTrait>,

    response_transformer: Arc<ResponseToJsonTransformer>,

    error_handler: Arc<ErrorHandler>,

    logger: Arc<dyn LoggerTrait>,
}

impl Server {
    pub fn new(
        addr: SocketAddr,
        router: Arc<dyn RouterTrait>,
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
            router: Arc<dyn RouterTrait>,
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
            let io = TokioIo::new(stream);

            let router = self.router.clone();
            let transformer = self.response_transformer.clone();
            let error_handler = self.error_handler.clone();
            let logger = self.logger.clone();

            tokio::task::spawn(async move {
                let start = Instant::now();
                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        io,
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

//ORIGINAL INTEGRATION https://github.com/hyperium/hyper-util/blob/master/src/rt/tokio_io.rs
//TODO: implement in differrent file or wait til hyper-util will be published into crates.io
/// Tokio IO integration for hyper
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;

pin_project! {
    /// A wrapping implementing hyper IO traits for a type that
    /// implements Tokio's IO traits.
    #[derive(Debug)]
    pub struct TokioIo<T> {
        #[pin]
        inner: T,
    }
}

impl<T> TokioIo<T> {
    /// Wrap a type implementing Tokio's IO traits.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> hyper::rt::Read for TokioIo<T>
where
    T: tokio::io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: hyper::rt::ReadBufCursor<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let n = unsafe {
            let mut tbuf = tokio::io::ReadBuf::uninit(buf.as_mut());
            match tokio::io::AsyncRead::poll_read(self.project().inner, cx, &mut tbuf) {
                Poll::Ready(Ok(())) => tbuf.filled().len(),
                other => return other,
            }
        };

        unsafe {
            buf.advance(n);
        }
        Poll::Ready(Ok(()))
    }
}

impl<T> hyper::rt::Write for TokioIo<T>
where
    T: tokio::io::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        tokio::io::AsyncWrite::poll_write(self.project().inner, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        tokio::io::AsyncWrite::poll_flush(self.project().inner, cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        tokio::io::AsyncWrite::poll_shutdown(self.project().inner, cx)
    }

    fn is_write_vectored(&self) -> bool {
        tokio::io::AsyncWrite::is_write_vectored(&self.inner)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<Result<usize, std::io::Error>> {
        tokio::io::AsyncWrite::poll_write_vectored(self.project().inner, cx, bufs)
    }
}
