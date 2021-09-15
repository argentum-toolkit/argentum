mod di;
mod web_app;

use crate::di::di_factory;
use crate::web_app::start_server;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let di = di_factory();
    start_server(di).await
}
