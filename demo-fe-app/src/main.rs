#![allow(non_snake_case)]

pub(crate) mod route;
pub(crate) mod rsx;
pub(crate) mod server;
pub(crate) mod user;
pub(crate) mod user_account;

use crate::rsx::app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing;

fn main() {
    // Init logger
    dioxus_sdk::storage::set_dir!();
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}
