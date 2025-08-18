use crate::rsx::page::user_account::{Login, Registration};
use crate::rsx::page::{Blog, Home};
use crate::rsx::wrapper::Wrapper;
use argentum_standard_ui::rsx::page::NotFound;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[nest("/user-account")]
            #[route("/login")]
            Login {},
            #[route("/register")]
            Registration {},
        #[end_nest]
        #[route("/blog/:id")]
        Blog { id: i32 },
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
