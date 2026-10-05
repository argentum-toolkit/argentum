mod di;

use crate::di::di_factory;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    argentum_standard_business::data_type::EMAIL_REGEX.is_match(""); //warmup regex

    let server = di_factory()?.server;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(server.serve())
}
