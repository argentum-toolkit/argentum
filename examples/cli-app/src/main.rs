use crate::app::App;
use crate::di::di_factory;

pub mod app;
mod di;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let di = di_factory()?;

    let app = App::new(
        di.id_factory.clone(),
        di.guest_registers_uc.clone(),
        di.member_logins_with_pw.clone(),
        di.member_registers_with_pw.clone(),
        di.user_authenticates_with_token.clone(),
        di.logger.clone(),
    );

    Ok(app.run().await?)
}
