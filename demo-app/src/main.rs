use crate::app::App;
use crate::di::di_factory;

pub mod app;
mod di;

fn main() -> Result<(), String> {
    let di = di_factory();
    let app = App::new(
        di.id_factory.clone(),
        di.anonymous_registers_uc.clone(),
        di.user_logins_with_pw.clone(),
        di.user_registers_with_pw.clone(),
        di.user_authenticates_with_token.clone(),
        di.logger.clone(),
    );

    app.run()
}
