use crate::di::di_factory;

pub mod app;
mod di;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = di_factory()?.app;

    Ok(app.run().await?)
}
