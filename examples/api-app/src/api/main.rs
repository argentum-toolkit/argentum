mod di;

use crate::di::di_factory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    argentum_standard_business::data_type::EMAIL_REGEX.is_match(""); //warmup regex

    let server = di_factory().await?;

    server.serve().await
}
