mod di;

use crate::di::di_factory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let migrators = di_factory().await?;

    for migrator in migrators.iter() {
        migrator.migrate().await?;
    }

    Ok(())
}
