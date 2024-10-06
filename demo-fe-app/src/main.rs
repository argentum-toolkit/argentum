#![allow(non_snake_case)]

pub(crate) mod route;
pub(crate) mod rsx;
pub(crate) mod server;
pub(crate) mod user;
pub(crate) mod user_account;

use crate::rsx::app::App;
use dioxus::fullstack::Config;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_logger::tracing::instrument::WithSubscriber;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    // Init logger
    dioxus_sdk::storage::set_dir!();
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");

    #[cfg(feature = "web")]
    launch(App);

    // #[cfg(feature = "web")]
    // // Hydrate the application on the client
    // dioxus_web::launch::launch_cfg(app, dioxus_web::Config::new().hydrate(true));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 9090));
    #[cfg(feature = "server")]
    {
        LaunchBuilder::new()
            .with_cfg(Config::new().addr(addr))
            .launch(App);
    }
}
