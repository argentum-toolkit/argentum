mod di;

use crate::di::di_factory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let di = di_factory();
    di.server.serve().await
    // start_server(di).await
}
