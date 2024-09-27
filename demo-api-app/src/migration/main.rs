mod di;

use crate::di::di_factory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let di = di_factory().await;

    di.user_migrator.migrate().await?;
    di.user_application_migrator.migrate().await?;

    Ok(())
}
