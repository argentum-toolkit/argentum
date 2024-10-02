use crate::rsx::blog::Blog;
use crate::rsx::home::Home;
use crate::rsx::page_not_found::PageNotFound;
use crate::rsx::wrapper::Wrapper;
use crate::user_account::rsx::login::Login;
use crate::user_account::rsx::registration::Registration;
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
    PageNotFound { route: Vec<String> },
}
