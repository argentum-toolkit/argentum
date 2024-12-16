#![allow(non_snake_case)]

pub(crate) mod route;
pub(crate) mod rsx;
pub(crate) mod standard;
pub(crate) mod user;
pub(crate) mod user_account;

fn main() {
    use crate::rsx::app::App;
    dioxus::launch(App);
}
