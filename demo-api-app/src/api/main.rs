mod di;

use crate::di::di_factory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    argentum_standard_business::data_type::email::EMAIL_REGEX.is_match(""); //warmup regex

    let di = di_factory().await?;
    di.server.serve().await
}
