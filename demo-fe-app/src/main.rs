#![allow(non_snake_case)]

pub(crate) mod route;
pub(crate) mod rsx;
pub(crate) mod server;

use crate::route::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing;

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
