#![allow(non_snake_case)]

pub(crate) mod route;
pub(crate) mod rsx;
pub(crate) mod standard;
pub(crate) mod user;
pub(crate) mod user_account;

use dioxus::prelude::*;
use dioxus_logger::tracing;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    // Init logger
    dioxus_sdk::storage::set_dir!();
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");

    #[cfg(feature = "web")]
    {
        use crate::rsx::app::App;
        launch(App);
    }

    #[cfg(feature = "server")]
    {
        use crate::rsx::app::App;
        use dioxus::fullstack::Config;

        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 9090));
        LaunchBuilder::new()
            .with_cfg(Config::new().addr(addr))
            .launch(App);
    }
}
